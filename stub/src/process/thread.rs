use std::ffi::c_void;
use std::ptr::null_mut;
use std::mem::transmute;

use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Threading::LPTHREAD_START_ROUTINE;

use crate::common::utils::NULL;
use crate::{debug, tartarusgate};

pub fn create_remote_thread(process_handle: HANDLE, start_address: usize, parameter: usize) -> Result<HANDLE, Box<dyn std::error::Error>> {
    unsafe {
        let mut t_handle: *mut c_void = null_mut();
        let start_routine: LPTHREAD_START_ROUTINE = Some(transmute(start_address));
        let start_routine_ptr = start_routine.unwrap() as usize;

        let status = tartarusgate!("NtCreateThreadEx", &mut t_handle as *mut _ as usize, 0x1FFFFF, NULL, process_handle.0 as usize, start_routine_ptr, parameter, 0, 0, 0, 0, 0);
        if status != 0 {
            return Err("No se pudo crear el hilo remoto".into());
        }
        debug!("Status de NtCreateThreadEx: {status:X}");
        debug!("Hilo del proceso nuevo: {t_handle:?}");

        Ok(HANDLE(t_handle as *mut _))
    }
}
