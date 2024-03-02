use core::arch::asm;

use super::Arch;

const KERNEL_BASE_ADDR: u64 = 0xffff800000000000;

fn paddr2ptr(paddr: u64) -> *mut u64 {
    (KERNEL_BASE_ADDR + paddr) as *mut u64
}

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

    fn mp_self() -> u32 {
        // FIXME:
        // https://github.com/nuta/resea/blob/3dbbcd9403abff70afb9df4573c8045e2146c6f7/kernel/arch/x64/arch.h#L67
        (unsafe { *(paddr2ptr(0xfee00020) as *const u32) } >> 24)
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
