use md5;
use rs_sha256::{HasherContext, Sha256Hasher};
use std::hash::Hasher;
pub fn md5_encode(decode: String) -> String {
    let digest = md5::compute(decode);
    format!("{:x}", digest)
}

pub fn md5_encode_upper(decode: String) -> String {
    let digest = md5::compute(decode);
    format!("{:x}", digest).to_uppercase()
}

pub fn md5_compress(decode: String) -> String {
    let encode = md5_encode(decode);
    let sub_str = &encode[0..8];
    sub_str.to_string()
}

pub fn sha2_256_encode(decode: String) -> String {
    let mut encoder = Sha256Hasher::default();
    encoder.write(decode.as_bytes());
    let encoder_byte = HasherContext::finish(&mut encoder);
    let encoder_str = format!("{:02x}", encoder_byte);
    encoder_str
}

pub fn sha2_256_encode_upper(decode: String) -> String {
    sha2_256_encode(decode).to_uppercase()
}
#[cfg(test)]
mod tests {
    use crate::{
        crypto::{md5_compress, md5_encode, md5_encode_upper},
        sha2_256_encode,
    };

    #[test]
    fn test_md5_encode() {
        let md = md5_encode("1".to_string());
        assert_eq!(md.as_str(), "c4ca4238a0b923820dcc509a6f75849b");
        let md = md5_encode_upper("1".to_string());
        assert_eq!(
            md.as_str(),
            "c4ca4238a0b923820dcc509a6f75849b".to_uppercase()
        );
        md5_compress("".to_string());
    }

    #[test]
    fn test_sha2_256_encode() {
        let encoder_str = sha2_256_encode("1".to_string());
        println!("{}", encoder_str);
    }
}
