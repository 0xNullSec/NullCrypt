pub const NULL: usize = 0;
pub const UP: isize = -32;
pub const DOWN: isize = 32;
pub const MAX_INSTRUCTION_CHECK: usize = 500;

pub fn align_page(size: usize) -> usize {
    (size + 0xFFF) & !0xFFF
}
