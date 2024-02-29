use core::arch::asm;

use super::Arch;

pub type X64 = ();

impl Arch for X64 {
    fn idle() -> ! {
        // FIXME: need lock
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
        // FIXME: need lock
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

    fn get_arch_cpuvar_mut() -> &'static mut super::CpuVar {
        let gsbase: u64;
        unsafe {
            asm! {
                "rdgsbase {0:e}",
                out(reg) gsbase,
            };
        }
        unsafe { &mut *(gsbase as *mut super::CpuVar) }
    }

    fn arch_init_per_cpu() {
        // https://github.com/nuta/microkernel-book/blob/2a49c4a932208ae22c0727cdd2047bf277bf447b/kernel/riscv32/setup.c#L163
        // TODO:
        let cpuvar = Self::get_arch_cpuvar_mut();
        cpuvar.id = 0;
        cpuvar.online = true;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ArchTask {}

#[derive(Debug, Clone, Copy)]
pub struct ArchVm {}

// TODO: assert sizeof(CpuVar) < 0x4000
#[derive(Debug, Clone, Copy)]
pub struct ArchCpuVar {}

fn asm_rdgsbase() -> u64 {
    let gsbase: u64;
    unsafe {
        asm! {
            "rdgsbase {0}",
            out(reg) gsbase,
        };
    }
    gsbase
}

fn asm_wrgsbase(gsbase: u64) {
    unsafe {
        asm! {
            "wrgsbase {0}",
            in(reg) gsbase,
        };
    }
}
