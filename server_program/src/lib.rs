use base64;

pub fn decode_from_base64(encoded_data: &str) -> Vec<u8> {
    base64::decode(encoded_data).expect("Error decoding from Base64")
}
