use crate::kernel::arch;
use log::info;

pub fn kernel_main() -> ! {
    info!("kernel_main");

    info!("CPL: {}", arch::get_cpl());

    arch::idle();
}
