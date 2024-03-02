use crate::build_config::NUM_CPU_MAX;

use self::x64::X64;

use super::task::{Task, TaskRef};

mod x64;

pub trait Arch {
    fn idle() -> !;
    fn get_cpl() -> u8;
    fn get_arch_cpuvar_mut() -> &'static mut CpuVar;
    fn arch_init_per_cpu() -> ();
    fn mp_self() -> u32;
}

pub fn idle() -> ! {
    #[cfg(target_arch = "x86_64")]
    X64::idle();
}

pub fn get_cpl() -> u8 {
    #[cfg(target_arch = "x86_64")]
    X64::get_cpl()
}

pub fn get_arch_cpuvar_mut() -> &'static mut CpuVar {
    #[cfg(target_arch = "x86_64")]
    X64::get_arch_cpuvar_mut()
}

pub fn arch_init_per_cpu() {
    #[cfg(target_arch = "x86_64")]
    X64::arch_init_per_cpu();
}

pub fn mp_self() -> u32 {
    #[cfg(target_arch = "x86_64")]
    X64::mp_self()
}

#[cfg(target_arch = "x86_64")]
pub type ArchTask = x64::ArchTask;

#[cfg(target_arch = "x86_64")]
pub type ArchVm = x64::ArchVm;

#[cfg(target_arch = "x86_64")]
pub type ArchCpuVar = x64::ArchCpuVar;

static mut CPUVARS: [CpuVar; NUM_CPU_MAX] = [CpuVar {
    arch: ArchCpuVar {},
    id: 0,
    online: false,
    current_task: Task::dummy().as_ref(),
    idle_task: Task::dummy(),
}; NUM_CPU_MAX];

#[derive(Debug, Clone, Copy)]
pub struct CpuVar {
    pub arch: ArchCpuVar,
    pub id: usize,
    pub online: bool,
    pub current_task: TaskRef,
    pub idle_task: Task,
}
