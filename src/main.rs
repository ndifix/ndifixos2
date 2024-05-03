#![no_std]
#![no_main]

use uefi::prelude::*;
use core::panic;
use core::fmt::Write;

#[panic_handler]
fn panic(_info: &panic::PanicInfo) -> ! {
  loop {}
}

#[entry]
fn efi_main(_handle: Handle, mut st: SystemTable<Boot>) -> Status {

  writeln!(st.stdout(), "Hello, world!").unwrap();

  loop {}
}
