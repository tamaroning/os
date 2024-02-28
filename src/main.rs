#![no_main]
#![no_std]

mod kernel;

use uefi::prelude::*;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    kernel::kernel_main()
}
