use log::info;

use super::kmain::kernel_main;

pub fn boot() -> ! {
    info!("Booting the kernel..");
    // TODO: CPUの初期化
    // x64
    // https://github.com/nuta/resea/blob/3dbbcd9403abff70afb9df4573c8045e2146c6f7/kernel/arch/x64/init.c#L223
    // riscv
    // https://github.com/nuta/microkernel-book/blob/2a49c4a932208ae22c0727cdd2047bf277bf447b/kernel/riscv32/setup.c#L84

    kernel_main();
}
