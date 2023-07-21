use crate::imports;
use crate::ports;

pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 25;

pub enum VGAColor {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGrey,
    DarkGrey,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightMagenta,
    LightBrown,
    White
}

pub trait Printable {
    fn print(&self, terminal: &Terminal);
}

impl Printable for &str {
    fn print(&self, terminal: &Terminal) {
        for i in 0..self.len() {
            let c = self.as_bytes()[i];
            terminal.putchar(c);
        }
    }
}

impl Printable for &[u8] {
    fn print(&self, terminal: &Terminal) {
        for c in self.iter() {
            terminal.putchar(*c);
        }
    }
}

#[repr(C)]
pub struct Terminal {
    buffer: *const u16,
    column: usize,
    row: usize,
    color: u8
}

impl Terminal {
    pub fn new() -> Self {
        unsafe {
            imports::terminal_init()
        }
    }

    pub fn get_pos(&self) -> (usize, usize) {
        (self.column, self.row)
    }

    pub fn putchar(&self, c: u8) {
        unsafe {
            imports::terminal_putchar(c, self);
        }
    }

    pub fn print(&self, s: impl Printable) {
        s.print(self);
    }

    pub fn println(&self, s: impl Printable) {
        self.print(s);
        self.putchar(b'\n');
    }

    pub fn clear(&self) {
        unsafe {
            imports::terminal_clear(self);
        }
    }

    pub fn disable_cursor(&self) {
        unsafe {
            ports::outb(0x3D4, 0x0A);
            ports::outb(0x3D5, 0x20);
        }
    }

    pub fn setcolor(&self, fg: VGAColor, bg: VGAColor) {
        unsafe  {
            imports::terminal_setcolor(fg as u8, bg as u8, self);
        }
    }

    pub fn setcursor(&self, x: usize, y: usize) {
        unsafe {
            imports::terminal_setcursor(x, y, self);
        }
    }

    pub fn unsetcursor(&self, x: usize, y: usize) {
        unsafe {
            imports::terminal_unsetcursor(x, y, self);
        }
    }

    pub fn delete(&self) {
        unsafe {
            imports::terminal_delete(self);
        }
    }
}
