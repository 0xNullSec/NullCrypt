use std::ffi::c_void;

#[repr(C, align(16))]
pub struct CryptoConfig {
    pub marker: [u8; 32],
    pub key: [u8; 32],
    pub nonce: [u8; 12],
    pub payload_size: u32,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct Vx_Table_Entry {
    pub pAddress: *mut c_void,
    pub dwHash: u64,
    pub wSystemCall: u16,
}

#[repr(C)]
pub struct Vx_Table {
    pub functions: Vx_Table_Entry,
}
