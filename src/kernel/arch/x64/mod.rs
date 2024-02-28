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

    fn get_cpl() -> u8 {
        let cpl: u8;
        unsafe {
            asm! {
                "mov ax, cs",
                "and ax, 3",
                "mov {0}, al",
                out(reg_byte) cpl,
            };
        }
        cpl
    }
}
