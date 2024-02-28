use self::x64::X64;

mod x64;

pub trait Arch {
    fn idle() -> !;
}

pub fn idle() -> ! {
    #[cfg(target_arch = "x86_64")]
    X64::idle();
}
