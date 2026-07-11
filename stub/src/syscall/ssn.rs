use std::ffi::c_void;
use std::ffi::CStr;
use std::ptr::null_mut;

use crate::common::structs::{Vx_Table, Vx_Table_Entry};
use crate::common::utils::{UP, DOWN, MAX_INSTRUCTION_CHECK};
use crate::syscall::exports::get_export_directory;
use crate::syscall::resolver::find_dll;
use crate::debug;

pub static mut VX_TABLE: Vx_Table = Vx_Table {
    functions: Vx_Table_Entry { pAddress: null_mut(), dwHash: 0, wSystemCall: 0 },
};

pub fn djb2(str: &[u8], mut dwhash: u64) -> Result<u64, Box<dyn std::error::Error>> {
    let mut i = 0;
    while i < str.len() {
        let c = str[i] as u64;
        dwhash = dwhash.wrapping_shl(5).wrapping_add(dwhash).wrapping_add(c);
        i += 1;
    }
    Ok(dwhash)
}

pub fn find_function(dll: *mut u8, pvx_table_entry: &mut Vx_Table_Entry, seed: u64) -> Result<u16, Box<dyn std::error::Error>> {
    unsafe {
        let export_directory = get_export_directory(dll)?;

        let name_rva = (*export_directory).AddressOfNames;
        let names = dll.offset(name_rva as isize) as *const u32;

        let ordinals_rva = (*export_directory).AddressOfNameOrdinals;
        let ordinals = dll.offset(ordinals_rva as isize) as *const u16;

        let functions_rva = (*export_directory).AddressOfFunctions;
        let functions = dll.offset(functions_rva as isize) as *const u32;

        for i in 0..(*export_directory).NumberOfNames {
            let name_rva = *names.offset(i as isize);
            let name_ptr = dll.offset(name_rva as isize) as *const i8;
            let cstr = CStr::from_ptr(name_ptr);
            let name_bytes: &[u8] = cstr.to_bytes();

            if djb2(name_bytes, seed)? == pvx_table_entry.dwHash {
                let ordinal = *ordinals.offset(i as isize);
                let func_rva = *functions.offset(ordinal as isize);
                let function_addr = dll.offset(func_rva as isize);
                debug!("Direccion memoria funcion: {:?}", function_addr);
                pvx_table_entry.pAddress = function_addr as *mut c_void;

                let ssn = extract_ssn(function_addr, pvx_table_entry)?;
                pvx_table_entry.wSystemCall = ssn;
                return Ok(ssn);
            }
        }
    }
    Ok(0)
}

pub fn extract_ssn(func_addr: *const u8, vx_table_entry: &mut Vx_Table_Entry) -> Result<u16, Box<dyn std::error::Error>> {
    unsafe {
        let mut cw = 0;
        let code = func_addr.cast::<u8>();

        loop {
            let b = |off: usize| *code.add(cw + off);

            if b(0) == 0x0f && b(1) == 0x05 {
                return Err("Se rompio xd".into())
            }

            if b(0) == 0xc3 {
                return Err("Hay una syscall directamente".into())
            }

            if b(0) == 0x4C && b(1) == 0x8B && b(2) == 0xD1 &&
               b(3) == 0xB8 && b(6) == 0x00 && b(7) == 0x00 {
                let high = b(5);
                let low = b(4);
                let ssn = ((high as u16) << 8) | (low as u16);
                vx_table_entry.wSystemCall = ssn;
                return Ok(ssn);
            }

            if b(0) == 0xE9 {
                for idx in 1..=MAX_INSTRUCTION_CHECK {
                    let b = |off: isize| *code.offset(off + (idx as isize) * DOWN);
                    let c = |off: isize| *code.offset(off + (idx as isize) * UP);

                    if b(0) == 0x4C && b(1) == 0x8B && b(2) == 0xD1 &&
                       b(3) == 0xB8 && b(6) == 0x00 && b(7) == 0x00 {
                        let high = b(5);
                        let low = b(4);
                        let ssn = ((high as u16) << 8) | ((low as i16 - idx as i16)) as u16;
                        vx_table_entry.wSystemCall = ssn;
                        return Ok(ssn);
                    }

                    if c(0) == 0x4C && c(1) == 0x8B && c(2) == 0xD1 &&
                       c(3) == 0xB8 && c(6) == 0x00 && c(7) == 0x00 {
                        let high = c(5);
                        let low = c(4);
                        let ssn = ((high as u16) << 8) | ((low as i16 + idx as i16)) as u16;
                        vx_table_entry.wSystemCall = ssn;
                        return Ok(ssn);
                    }
                }
            }

            if b(3) == 0xE9 {
                for idx in 1..=MAX_INSTRUCTION_CHECK {
                    let b = |off: isize| *code.offset(off + (idx as isize) * DOWN);
                    let c = |off: isize| *code.offset(off + (idx as isize) * UP);

                    if b(0) == 0x4C && b(1) == 0x8B && b(2) == 0xD1 &&
                       b(3) == 0xB8 && b(6) == 0x00 && b(7) == 0x00 {
                        let high = b(5);
                        let low = b(4);
                        let ssn = ((high as u16) << 8) | ((low as i16 - idx as i16)) as u16;
                        vx_table_entry.wSystemCall = ssn;
                        return Ok(ssn);
                    }

                    if c(0) == 0x4C && c(1) == 0x8B && c(2) == 0xD1 &&
                       c(3) == 0xB8 && c(6) == 0x00 && c(7) == 0x00 {
                        let high = c(5);
                        let low = c(4);
                        let ssn = ((high as u16) << 8) | ((low as i16 + idx as i16)) as u16;
                        vx_table_entry.wSystemCall = ssn;
                        return Ok(ssn);
                    }
                }
            }

            cw += 1
        }
    }
    #[allow(unused)]
    Err("No se encontro la SSN".into())
}

pub fn get_ssn(api_hash: u64, seed: u64) -> Result<u16, Box<dyn std::error::Error>> {
    let table = &raw mut VX_TABLE;

    let locate_dll = "ntdll.dll";
    let address_dll = find_dll(locate_dll)?;

    let ssn = unsafe {
        (*table).functions.dwHash = api_hash;
        find_function(address_dll as *mut u8, &mut (*table).functions, seed)?
    };

    Ok(ssn)
}
