use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NsqSyntaxNode {
    pub kind: String,
    pub start: usize,
    pub end: usize,
    pub children: Vec<NsqSyntaxNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NsqSyntaxTree {
    pub source_len: usize,
    pub root: NsqSyntaxNode,
}

impl NsqSyntaxTree {
    pub fn parse(source: &str) -> Result<Self, String> {
        let bytes = source.as_bytes();
        let mut roots = Vec::new();
        let mut stack: Vec<NsqSyntaxNode> = Vec::new();
        let mut token_start: Option<usize> = None;

        for (index, byte) in bytes.iter().copied().enumerate() {
            match byte {
                b'(' | b'[' | b'{' => {
                    flush_token(&mut token_start, index, &mut stack, &mut roots);
                    stack.push(NsqSyntaxNode {
                        kind: (byte as char).to_string(),
                        start: index,
                        end: index + 1,
                        children: Vec::new(),
                    });
                }
                b')' | b']' | b'}' => {
                    flush_token(&mut token_start, index, &mut stack, &mut roots);
                    let expected = matching_open(byte);
                    let Some(mut node) = stack.pop() else {
                        return Err(format!("unexpected closing delimiter at byte {index}"));
                    };
                    if node.kind != expected.to_string() {
                        return Err(format!("mismatched delimiter at byte {index}"));
                    }
                    node.end = index + 1;
                    attach(node, &mut stack, &mut roots);
                }
                byte if byte.is_ascii_whitespace() || b";,:".contains(&byte) => {
                    flush_token(&mut token_start, index, &mut stack, &mut roots);
                }
                _ => {
                    if token_start.is_none() {
                        token_start = Some(index);
                    }
                }
            }
        }
        flush_token(&mut token_start, bytes.len(), &mut stack, &mut roots);
        if let Some(node) = stack.last() {
            return Err(format!(
                "unclosed delimiter `{}` at byte {}",
                node.kind, node.start
            ));
        }
        Ok(Self {
            source_len: bytes.len(),
            root: NsqSyntaxNode {
                kind: "root".into(),
                start: 0,
                end: bytes.len(),
                children: roots,
            },
        })
    }

    pub fn replace_range(
        &self,
        source: &str,
        start: usize,
        end: usize,
        replacement: &str,
    ) -> Result<Self, String> {
        if start > end
            || end > source.len()
            || !source.is_char_boundary(start)
            || !source.is_char_boundary(end)
        {
            return Err("syntax replacement range is invalid".into());
        }
        let mut updated = String::with_capacity(source.len() - (end - start) + replacement.len());
        updated.push_str(&source[..start]);
        updated.push_str(replacement);
        updated.push_str(&source[end..]);
        Self::parse(&updated)
    }
}

fn flush_token(
    token_start: &mut Option<usize>,
    end: usize,
    stack: &mut Vec<NsqSyntaxNode>,
    roots: &mut Vec<NsqSyntaxNode>,
) {
    if let Some(start) = token_start.take() {
        if start < end {
            let node = NsqSyntaxNode {
                kind: "atom".into(),
                start,
                end,
                children: Vec::new(),
            };
            attach(node, stack, roots);
        }
    }
}

fn attach(node: NsqSyntaxNode, stack: &mut Vec<NsqSyntaxNode>, roots: &mut Vec<NsqSyntaxNode>) {
    if let Some(parent) = stack.last_mut() {
        parent.children.push(node);
    } else {
        roots.push(node);
    }
}

fn matching_open(close: u8) -> char {
    match close {
        b')' => '(',
        b']' => '[',
        b'}' => '{',
        _ => '?',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_nested_source_with_stable_spans() {
        let tree = NsqSyntaxTree::parse("fn main() { return (x + 1); }").unwrap();
        assert_eq!(tree.root.kind, "root");
        assert_eq!(tree.source_len, 29);
        assert!(tree
            .root
            .children
            .iter()
            .any(|node| node.kind == "{" && node.end > node.start));
    }

    #[test]
    fn rejects_unbalanced_source_fail_closed() {
        assert!(NsqSyntaxTree::parse("fn main( {").is_err());
        assert!(NsqSyntaxTree::parse("fn main())").is_err());
    }

    #[test]
    fn replacement_rebuilds_only_from_raw_source_intent() {
        let tree = NsqSyntaxTree::parse("(old)").unwrap();
        let updated = tree.replace_range("(old)", 1, 4, "new").unwrap();
        assert_eq!(updated.root.children[0].children[0].start, 1);
        assert_eq!(updated.root.children[0].children[0].end, 4);
    }
}
