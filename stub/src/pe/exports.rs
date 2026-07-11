use std::ffi::CStr;

use windows::Win32::System::SystemServices::{
    IMAGE_DOS_HEADER, IMAGE_EXPORT_DIRECTORY,
};
use windows::Win32::System::Diagnostics::Debug::{
    IMAGE_NT_HEADERS64, IMAGE_DIRECTORY_ENTRY_EXPORT,
};

use crate::syscall::resolver::find_dll;

pub fn get_export_directory(dll: *mut u8) -> Result<*const IMAGE_EXPORT_DIRECTORY, Box<dyn std::error::Error>> {
    unsafe {
        let dos_header = dll as *const IMAGE_DOS_HEADER;
        let nt_header = dll.add((*dos_header).e_lfanew as usize) as *const IMAGE_NT_HEADERS64;
        let export_directory_rva = (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT.0 as usize].VirtualAddress;
        Ok(dll.add(export_directory_rva as usize) as *const IMAGE_EXPORT_DIRECTORY)
    }
}

pub fn find_loadlibrary(dll: *mut u8, function_name: &str) -> Result<*mut u8, Box<dyn std::error::Error>> {
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
            let name = CStr::from_ptr(name_ptr).to_str()?;

            if name == function_name {
                let ordinal = *ordinals.offset(i as isize);
                let func_rva = *functions.offset(ordinal as isize);
                return Ok(dll.offset(func_rva as isize))
            }
        }
    }
    Ok(0 as *mut u8)
}

pub fn searcher_loadlibrary(dll_name: &str, function_call: &str) -> Result<*mut u8, Box<dyn std::error::Error>> {
    let address_dll: *mut u8 = find_dll(dll_name)?;
    let address_function = find_loadlibrary(address_dll, function_call)?;
    Ok(address_function)
}
