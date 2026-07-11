use crate::common::structs::CryptoConfig;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".reloc")]
pub static mut CONFIG: CryptoConfig = CryptoConfig {
    marker: *b"___THIS_IS_A_PLACEHOLDER_MARK___",
    key: [0u8; 32],
    nonce: [0u8; 12],
    payload_size: 0,
};
