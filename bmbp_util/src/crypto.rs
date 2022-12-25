use md5;

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

#[cfg(test)]
mod tests {
    use crate::crypto::{md5_compress, md5_encode, md5_encode_upper};

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
}
