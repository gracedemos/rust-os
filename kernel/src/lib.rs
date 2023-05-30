#![no_std]

mod imports;
mod terminal;

use core::panic::PanicInfo;
use terminal::Terminal;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let terminal = Terminal::new();
    terminal.println("hehe");
    terminal.print("bruh");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
