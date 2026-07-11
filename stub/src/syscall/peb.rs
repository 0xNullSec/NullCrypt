use std::arch::asm;
use windows::Win32::System::Threading::PEB;

pub fn get_peb() -> Result<*mut PEB, Box<dyn std::error::Error>> {
    let mut peb: *mut PEB;
    unsafe {
        asm!(
            "mov {peb_out}, gs:[0x60]",
            peb_out = out(reg) peb,
        );
    }
    Ok(peb)
}
