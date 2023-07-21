#![no_std]

mod imports;
mod terminal;
mod ports;
mod menu;
mod shell;
mod command;

use core::panic::PanicInfo;

use terminal::{Terminal, VGAColor};
use menu::Menu;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let terminal = Terminal::new();

    let mut menu = Menu::new(&terminal);
    menu.run();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
