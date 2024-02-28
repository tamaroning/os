use core::arch::asm;

use super::Arch;

pub type X64 = ();

impl Arch for X64 {
    fn idle() -> ! {
        loop {
            unsafe {
                asm! {
                    "sti",
                    "hlt",
                    "cli"
                };
            }
        }
    }
}
