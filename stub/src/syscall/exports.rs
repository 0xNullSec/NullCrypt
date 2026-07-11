use windows::Win32::System::SystemServices::{
    IMAGE_DOS_HEADER, IMAGE_EXPORT_DIRECTORY,
};
use windows::Win32::System::Diagnostics::Debug::{
    IMAGE_NT_HEADERS64, IMAGE_DIRECTORY_ENTRY_EXPORT,
};

pub fn get_export_directory(dll: *mut u8) -> Result<*const IMAGE_EXPORT_DIRECTORY, Box<dyn std::error::Error>> {
    unsafe {
        let dos_header = dll as *const IMAGE_DOS_HEADER;
        let nt_header = dll.add((*dos_header).e_lfanew as usize) as *const IMAGE_NT_HEADERS64;
        let export_directory_rva = (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT.0 as usize].VirtualAddress;
        Ok(dll.add(export_directory_rva as usize) as *const IMAGE_EXPORT_DIRECTORY)
    }
}
