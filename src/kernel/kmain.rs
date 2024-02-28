use log::info;

use crate::kernel::arch;

pub fn kernel_main() -> ! {
    info!("Booting the kernel..");

    arch::idle();
}
