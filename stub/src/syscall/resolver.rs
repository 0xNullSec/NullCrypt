use std::mem::offset_of;

use windows::Win32::System::SystemServices::IMAGE_DOS_HEADER;
use windows::Win32::System::Diagnostics::Debug::{IMAGE_NT_HEADERS64, IMAGE_SECTION_HEADER};
use windows::Win32::System::WindowsProgramming::LDR_DATA_TABLE_ENTRY;

use crate::syscall::peb::get_peb;
use crate::debug;

pub fn find_dll(dll: &str) -> Result<*mut u8, Box<dyn std::error::Error>> {
    let peb = get_peb()?;
    unsafe {
        debug!("Direccion del PEB: {:?}", peb);

        let ldr = (*peb).Ldr;
        debug!("Direccion del LDR: {:?}", ldr);

        let list = &(*ldr).InMemoryOrderModuleList;
        let list_head = list as *const _ as usize;
        let mut link = list.Flink;

        while (link as usize) != list_head {
            let entry = (link as usize - offset_of!(LDR_DATA_TABLE_ENTRY, InMemoryOrderLinks)) as *mut LDR_DATA_TABLE_ENTRY;
            let base = (*entry).DllBase;
            let name = (*entry).FullDllName;
            let name_buff = std::slice::from_raw_parts(name.Buffer.0, (name.Length / 2) as usize);
            let name_str = String::from_utf16_lossy(name_buff);

            if name_str.to_lowercase().contains(dll) {
                debug!("Puntero de {:?}: {:?}", dll, base);
                return Ok(base as *mut u8)
            }

            link = (*link).Flink
        }
    }
    Err("No se encontro el puntero".into())
}

pub fn find_syscall_ntdll(ntdll: *mut u8) -> Result<*mut u8, Box<dyn std::error::Error>> {
    unsafe {
        let dos = ntdll as *const IMAGE_DOS_HEADER;
        let nt = ntdll.add((*dos).e_lfanew as usize) as *const IMAGE_NT_HEADERS64;

        let sections = (nt as usize + std::mem::size_of::<IMAGE_NT_HEADERS64>())
            as *const IMAGE_SECTION_HEADER;

        for i in 0..(*nt).FileHeader.NumberOfSections {
            let sec = &*sections.add(i as usize);
            let name = std::slice::from_raw_parts(sec.Name.as_ptr(), 8);
            if name.starts_with(b".text") {
                let base = ntdll.add(sec.VirtualAddress as usize);
                let size = sec.Misc.VirtualSize as usize;

                for j in 0..size {
                    let ptr = base.add(j);
                    if *ptr == 0x0F &&
                       *ptr.add(1) == 0x05 &&
                       *ptr.add(2) == 0xC3 {
                        return Ok(ptr as *mut u8);
                    }
                }
            }
        }
    }
    Ok(std::ptr::null_mut())
}
