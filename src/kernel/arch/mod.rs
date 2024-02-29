use self::x64::X64;

mod x64;

pub trait Arch {
    fn idle() -> !;
    fn get_cpl() -> u8;
}

pub fn idle() -> ! {
    #[cfg(target_arch = "x86_64")]
    X64::idle();
}

pub fn get_cpl() -> u8 {
    #[cfg(target_arch = "x86_64")]
    X64::get_cpl()
}

#[cfg(target_arch = "x86_64")]
pub type ArchTask = x64::ArchTask;

#[cfg(target_arch = "x86_64")]
pub type ArchVm = x64::ArchVm;

pub struct CpuVar {
    pub arch: ArchCpuVar,
}
