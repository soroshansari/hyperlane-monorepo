use ethers::utils::hex;

pub fn convert_address_to_bytes32(address: &str) -> [u8; 32] {
    // Remove the "0x" prefix if present
    let address = if address.starts_with("0x") {
        &address[2..]
    } else {
        address
    };

    // Decode the hex-encoded string into a Vec<u8>
    let bytes = hex::decode(address).expect("Failed to decode address");

    // Check the length of the bytes vector
    if bytes.len() != 20 {
        panic!("Invalid address length");
    }

    // Convert Vec<u8> to [u8; 32]
    let mut address_bytes: [u8; 32] = [0; 32];
    address_bytes[..20].copy_from_slice(&bytes);

    address_bytes
}

pub fn string_to_hex_bytes(string: &str) -> Vec<u8> {
    // Convert string to bytes
    let bytes = string.as_bytes();

    // Convert bytes to hex-encoded string
    let hex_string = hex::encode(bytes);

    // Convert hex-encoded string to Vec<u8>
    hex::decode(hex_string).expect("Failed to decode hex string")
}
