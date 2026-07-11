#[unsafe(no_mangle)]
pub static mut pSyscallStub: *mut u8 = std::ptr::null_mut();

pub static mut SYSCALL_STUB: *mut u8 = std::ptr::null_mut();

unsafe extern "C" {
    #[link_name = "HellsGate"]
    pub fn hells_gate(ssn: u16);

    #[link_name = "HellDescent"]
    pub fn hell_descent(arg1: usize, ...) -> i32;
}

#[macro_export]
macro_rules! tartarusgate {
    ($func_string:literal, $($arg:expr),* $(,)?) => {
        {
            let (hash_calc, seed) = hash_macro::calc_hash!($func_string);
            let ssn = $crate::syscall::ssn::get_ssn(hash_calc, seed)?;
            $crate::debug_plain!("SSN {:?}", ssn);
            $crate::syscall::hellgates::hells_gate(ssn);
            let status: i32 = $crate::syscall::hellgates::hell_descent($($arg as usize),*);
            status
        }
    };
}
