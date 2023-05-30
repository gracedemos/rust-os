use crate::imports;

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

    pub fn putchar(&self, c: u8) {
        unsafe {
            imports::terminal_putchar(c, self);
        }
    }

    pub fn print(&self, s: &str) {
        for i in 0..s.len() {
            let c = s.as_bytes()[i];
            self.putchar(c);
        }
    }

    pub fn println(&self, s: &str) {
        self.print(s);
        self.putchar(b'\n');
    }

    pub fn clear(&self) {
        unsafe {
            imports::terminal_clear(self);
        }
    }
}
