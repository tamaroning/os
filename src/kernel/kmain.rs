use crate::kernel::arch;
use log::info;

use super::task;

pub fn kernel_main() -> ! {
    info!("kernel_main");

    info!("CPL: {}", arch::get_cpl());

    // become idle task
    mp_kernel_main();
}


pub fn mp_kernel_main() -> ! {
    // https://github.com/nuta/resea/blob/3dbbcd9403abff70afb9df4573c8045e2146c6f7/kernel/boot.c#L151
    info!("Booting CPU");
    arch::arch_init_per_cpu();
    task::task_init_per_cpu();
    
    //let cpuvar = arch::get_arch_cpuvar_mut();
    //info!("Booted CPU#{}", cpuvar.id);
    info!("Booted CPU");
    arch::idle();
}
