use std::str::FromStr;

use nsq_compress::CompressionArch;

#[test]
fn parse_architectures() {
    assert_eq!(CompressionArch::from_str("nu256").unwrap(), CompressionArch::Nu256);
    assert_eq!(CompressionArch::from_str("nu336").unwrap(), CompressionArch::Nu336);
    assert_eq!(CompressionArch::from_str("nu369").unwrap(), CompressionArch::Nu369);
}
