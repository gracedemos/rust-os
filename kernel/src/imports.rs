use crate::terminal::Terminal;

extern "C" {
    pub fn terminal_init() -> Terminal;
    pub fn terminal_putchar(c: u8, terminal: &Terminal);
    pub fn terminal_clear(terminal: &Terminal);
    pub fn terminal_setcolor(fg: u8, bg: u8, terminal: &Terminal);
    pub fn terminal_setcursor(x: usize, y: usize, terminal: &Terminal);
    pub fn terminal_unsetcursor(x: usize, y: usize, terminal: &Terminal);
    pub fn terminal_delete(terminal: &Terminal);
}
