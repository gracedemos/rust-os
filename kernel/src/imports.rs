use crate::terminal::Terminal;

extern "C" {
    pub fn terminal_init() -> Terminal;
    pub fn terminal_putchar(c: u8, terminal: &Terminal);
    pub fn terminal_clear(terminal: &Terminal);
}
