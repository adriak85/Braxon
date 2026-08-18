use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

pub const NSQ_TENSOR_SCHEMA: &str = "nsq.native_tensor.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoritativeTensorDescriptor {
    pub name: String,
    pub dtype: String,
    pub shape: Vec<u64>,
    pub data_offset: u64,
    pub data_len: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NsqTensor {
    pub schema: String,
    pub name: String,
    pub dtype: String,
    pub shape: Vec<u64>,
    pub source_path: String,
    pub source_sha256: String,
    pub materialization_sha256: String,
    pub generation: u64,
    pub bytes: Vec<u8>,
}

#[derive(Debug)]
pub enum NativeTensorError {
    Io(io::Error),
    InvalidFormat(String),
    MissingTensor(String),
    UnsupportedDtype(String),
    ShapeMismatch { expected: usize, actual: usize },
}

impl std::fmt::Display for NativeTensorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "tensor io error: {error}"),
            Self::InvalidFormat(message) => write!(f, "invalid tensor format: {message}"),
            Self::MissingTensor(name) => write!(f, "missing tensor: {name}"),
            Self::UnsupportedDtype(dtype) => write!(f, "unsupported tensor dtype: {dtype}"),
            Self::ShapeMismatch { expected, actual } => {
                write!(
                    f,
                    "tensor shape mismatch: expected {expected} values, got {actual}"
                )
            }
        }
    }
}

impl std::error::Error for NativeTensorError {}

impl From<io::Error> for NativeTensorError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthoritativeModelIndex {
    pub weight_map: BTreeMap<String, String>,
}

impl AuthoritativeModelIndex {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, NativeTensorError> {
        let path = path.as_ref();
        let bytes = std::fs::read(path)?;
        let index: Self = serde_json::from_slice(&bytes)
            .map_err(|error| NativeTensorError::InvalidFormat(error.to_string()))?;
        if index.weight_map.is_empty() {
            return Err(NativeTensorError::InvalidFormat(
                "model index has an empty weight_map".into(),
            ));
        }
        Ok(index)
    }

    pub fn shard_for(&self, tensor_name: &str) -> Result<&str, NativeTensorError> {
        self.weight_map
            .get(tensor_name)
            .map(String::as_str)
            .ok_or_else(|| NativeTensorError::MissingTensor(tensor_name.to_owned()))
    }

    pub fn materialize_tensor(
        &self,
        index_path: impl AsRef<Path>,
        tensor_name: &str,
        window_bytes: usize,
        generation: u64,
    ) -> Result<NsqTensor, NativeTensorError> {
        let index_path = index_path.as_ref();
        let shard = self.shard_for(tensor_name)?;
        let shard_path = index_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(shard);
        if !shard_path.is_file() {
            return Err(NativeTensorError::Io(io::Error::new(
                io::ErrorKind::NotFound,
                format!("authoritative shard is absent: {}", shard_path.display()),
            )));
        }
        let mut reader = BoundedShardReader::open(shard_path, window_bytes)?;
        reader.materialize(tensor_name, generation)
    }
}

#[derive(Debug)]
pub struct BoundedShardReader {
    path: PathBuf,
    file: File,
    window_bytes: usize,
    max_window_read: usize,
    source_sha256: String,
    descriptors: BTreeMap<String, AuthoritativeTensorDescriptor>,
    data_start: u64,
}

impl BoundedShardReader {
    pub fn open(path: impl AsRef<Path>, window_bytes: usize) -> Result<Self, NativeTensorError> {
        if window_bytes == 0 {
            return Err(NativeTensorError::InvalidFormat(
                "window size must be non-zero".into(),
            ));
        }
        let path = path.as_ref().to_path_buf();
        let mut file = File::open(&path)?;
        let source_sha256 = hash_file(&mut file)?;
        file.seek(SeekFrom::Start(0))?;
        let header_len = read_u64_le(&mut file)?;
        if header_len > usize::MAX as u64 {
            return Err(NativeTensorError::InvalidFormat(
                "header exceeds addressable memory".into(),
            ));
        }
        let mut header = vec![0u8; header_len as usize];
        file.read_exact(&mut header)?;
        let value: serde_json::Value = serde_json::from_slice(&header)
            .map_err(|error| NativeTensorError::InvalidFormat(error.to_string()))?;
        let object = value.as_object().ok_or_else(|| {
            NativeTensorError::InvalidFormat("safetensors header is not an object".into())
        })?;
        let mut descriptors = BTreeMap::new();
        for (name, entry) in object {
            if name == "__metadata__" {
                continue;
            }
            let dtype = entry
                .get("dtype")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| NativeTensorError::InvalidFormat(format!("{name} has no dtype")))?;
            let shape = entry
                .get("shape")
                .and_then(serde_json::Value::as_array)
                .ok_or_else(|| NativeTensorError::InvalidFormat(format!("{name} has no shape")))?
                .iter()
                .map(|item| {
                    item.as_u64().ok_or_else(|| {
                        NativeTensorError::InvalidFormat(format!("{name} has invalid shape"))
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let offsets = entry
                .get("data_offsets")
                .and_then(serde_json::Value::as_array)
                .ok_or_else(|| {
                    NativeTensorError::InvalidFormat(format!("{name} has no data_offsets"))
                })?;
            if offsets.len() != 2 {
                return Err(NativeTensorError::InvalidFormat(format!(
                    "{name} data_offsets must have two values"
                )));
            }
            let start = offsets[0].as_u64().ok_or_else(|| {
                NativeTensorError::InvalidFormat(format!("{name} has invalid start offset"))
            })?;
            let end = offsets[1].as_u64().ok_or_else(|| {
                NativeTensorError::InvalidFormat(format!("{name} has invalid end offset"))
            })?;
            if end < start {
                return Err(NativeTensorError::InvalidFormat(format!(
                    "{name} has descending offsets"
                )));
            }
            descriptors.insert(
                name.clone(),
                AuthoritativeTensorDescriptor {
                    name: name.clone(),
                    dtype: dtype.to_owned(),
                    shape,
                    data_offset: start,
                    data_len: end - start,
                },
            );
        }
        Ok(Self {
            path,
            file,
            window_bytes,
            max_window_read: 0,
            source_sha256,
            descriptors,
            data_start: 8 + header_len,
        })
    }

    pub fn descriptor(
        &self,
        name: &str,
    ) -> Result<&AuthoritativeTensorDescriptor, NativeTensorError> {
        self.descriptors
            .get(name)
            .ok_or_else(|| NativeTensorError::MissingTensor(name.to_owned()))
    }

    pub fn source_sha256(&self) -> &str {
        &self.source_sha256
    }

    pub fn max_window_read(&self) -> usize {
        self.max_window_read
    }

    pub fn materialize(
        &mut self,
        name: &str,
        generation: u64,
    ) -> Result<NsqTensor, NativeTensorError> {
        let descriptor = self.descriptor(name)?.clone();
        let mut bytes = Vec::with_capacity(descriptor.data_len as usize);
        let mut remaining = descriptor.data_len;
        let mut offset = self.data_start + descriptor.data_offset;
        while remaining > 0 {
            let amount = remaining.min(self.window_bytes as u64) as usize;
            self.file.seek(SeekFrom::Start(offset))?;
            let mut window = vec![0u8; amount];
            self.file.read_exact(&mut window)?;
            self.max_window_read = self.max_window_read.max(window.len());
            bytes.extend_from_slice(&window);
            offset += amount as u64;
            remaining -= amount as u64;
        }
        let materialization_sha256 = hash_bytes(&bytes);
        Ok(NsqTensor {
            schema: NSQ_TENSOR_SCHEMA.into(),
            name: descriptor.name,
            dtype: descriptor.dtype,
            shape: descriptor.shape,
            source_path: self.path.display().to_string(),
            source_sha256: self.source_sha256.clone(),
            materialization_sha256,
            generation,
            bytes,
        })
    }
}

#[derive(Debug, Default)]
pub struct NsqTensorStore {
    tensors: BTreeMap<String, NsqTensor>,
}

impl NsqTensorStore {
    pub fn insert(&mut self, tensor: NsqTensor) -> Result<(), NativeTensorError> {
        if tensor.schema != NSQ_TENSOR_SCHEMA {
            return Err(NativeTensorError::InvalidFormat(
                "tensor does not use the canonical NSQ schema".into(),
            ));
        }
        self.tensors.insert(tensor.name.clone(), tensor);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<&NsqTensor, NativeTensorError> {
        self.tensors
            .get(name)
            .ok_or_else(|| NativeTensorError::MissingTensor(name.to_owned()))
    }

    pub fn parameter_dot(&self, name: &str, input: &[f32]) -> Result<f32, NativeTensorError> {
        let tensor = self.get(name)?;
        if tensor.dtype != "F32" {
            return Err(NativeTensorError::UnsupportedDtype(tensor.dtype.clone()));
        }
        if tensor.bytes.len() % 4 != 0 {
            return Err(NativeTensorError::InvalidFormat(
                "F32 tensor byte length is not divisible by four".into(),
            ));
        }
        let values = tensor
            .bytes
            .chunks_exact(4)
            .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect::<Vec<_>>();
        if values.len() != input.len() {
            return Err(NativeTensorError::ShapeMismatch {
                expected: values.len(),
                actual: input.len(),
            });
        }
        Ok(values
            .iter()
            .zip(input)
            .map(|(parameter, value)| parameter * value)
            .sum())
    }
}

fn read_u64_le(file: &mut File) -> Result<u64, NativeTensorError> {
    let mut bytes = [0u8; 8];
    file.read_exact(&mut bytes)?;
    Ok(u64::from_le_bytes(bytes))
}

fn hash_file(file: &mut File) -> Result<String, NativeTensorError> {
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_safetensors(path: &Path, values: &[f32]) {
        let data = values
            .iter()
            .flat_map(|value| value.to_le_bytes())
            .collect::<Vec<_>>();
        let header = format!(
            "{{\"weight\":{{\"dtype\":\"F32\",\"shape\":[{}],\"data_offsets\":[0,{}]}}}}",
            values.len(),
            data.len()
        );
        let mut file = File::create(path).unwrap();
        file.write_all(&(header.len() as u64).to_le_bytes())
            .unwrap();
        file.write_all(header.as_bytes()).unwrap();
        file.write_all(&data).unwrap();
    }

    #[test]
    fn bounded_reader_materializes_authoritative_tensor_and_tracks_window_limit() {
        let path = std::env::temp_dir().join(format!(
            "nsq-tensor-{}-{}.safetensors",
            std::process::id(),
            1
        ));
        write_safetensors(&path, &[1.0, 2.0, 3.0, 4.0]);
        let mut reader = BoundedShardReader::open(&path, 4).unwrap();
        let tensor = reader.materialize("weight", 1).unwrap();
        assert_eq!(tensor.shape, vec![4]);
        assert_eq!(tensor.bytes.len(), 16);
        assert_eq!(tensor.generation, 1);
        assert!(reader.max_window_read() <= 4);
        assert_eq!(tensor.source_sha256, reader.source_sha256());
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn rematerialization_changes_parameter_backed_computation() {
        let path = std::env::temp_dir().join(format!(
            "nsq-tensor-{}-{}.safetensors",
            std::process::id(),
            2
        ));
        write_safetensors(&path, &[1.0, 2.0]);
        let mut reader = BoundedShardReader::open(&path, 4).unwrap();
        let mut store = NsqTensorStore::default();
        store
            .insert(reader.materialize("weight", 1).unwrap())
            .unwrap();
        let first = store.parameter_dot("weight", &[3.0, 4.0]).unwrap();
        write_safetensors(&path, &[2.0, 4.0]);
        let mut refreshed = BoundedShardReader::open(&path, 4).unwrap();
        let tensor = refreshed.materialize("weight", 2).unwrap();
        assert_ne!(
            tensor.materialization_sha256,
            store.get("weight").unwrap().materialization_sha256
        );
        store.insert(tensor).unwrap();
        let second = store.parameter_dot("weight", &[3.0, 4.0]).unwrap();
        assert_eq!(first, 11.0);
        assert_eq!(second, 22.0);
        assert_ne!(first, second);
        assert_eq!(store.get("weight").unwrap().generation, 2);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn authoritative_index_fails_closed_when_declared_shard_is_absent() {
        let root = std::env::temp_dir().join(format!("nsq-index-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let index_path = root.join("model.safetensors.index.json");
        std::fs::write(
            &index_path,
            r#"{"weight_map":{"weight":"model-00001-of-00001.safetensors"}}"#,
        )
        .unwrap();
        let index = AuthoritativeModelIndex::from_path(&index_path).unwrap();
        let error = index
            .materialize_tensor(&index_path, "weight", 4, 1)
            .unwrap_err();
        assert!(error.to_string().contains("authoritative shard is absent"));
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn missing_store_link_fails_closed() {
        let store = NsqTensorStore::default();
        assert!(matches!(
            store.parameter_dot("missing", &[1.0]),
            Err(NativeTensorError::MissingTensor(_))
        ));
    }
}
