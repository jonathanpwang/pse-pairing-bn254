#![feature(asm)]
#![feature(asm_const)]

#[macro_use]
mod binops;
#[macro_use]
mod derive;

pub(crate) mod arithmetic;
pub mod bn254;
pub mod secp256k1;

pub extern crate group;

#[cfg(test)]
pub mod tests;

#[cfg(feature = "prefetch")]
#[inline(always)]
pub fn prefetch<T>(data: &[T], offset: usize) {
    use core::arch::x86_64::_mm_prefetch;
    unsafe {
        _mm_prefetch(
            data.as_ptr().offset(offset as isize) as *const i8,
            core::arch::x86_64::_MM_HINT_T0,
        );
    }
}
