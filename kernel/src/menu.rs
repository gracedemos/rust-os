use crate::terminal::{Terminal, VGAColor};
use crate::ports;
use crate::shell::Shell;

pub struct Menu<'a> {
    terminal: &'a Terminal,
    sel: u32,
}

impl<'a> Menu<'a> {
    pub fn new(terminal: &'a Terminal) -> Self {
        Menu {
            terminal,
            sel: 0
        }
    }

    pub fn run(&mut self) {
        self.init();

        let mut prev = 0;
        loop {
            prev = self.handle_events(prev);
        }
    }

    fn handle_events(&mut self, prev: u8) -> u8 {
        let scancode = unsafe {
            ports::inb(0x60)
        };

        if scancode == prev {
            return prev;
        }

        match scancode {
            0x50 => {
               if self.sel < 1 {
                   self.sel += 1;
                   self.init();
               }
            },
            0x48 => {
                if self.sel > 0 {
                    self.sel -= 1;
                    self.init();
                }
            },
            0x1C => {
                match self.sel {
                    0 => { // Shell
                        let mut shell = Shell::new(self.terminal);
                        shell.run();

                        self.init();
                    },
                    1 => { // Shut Down
                        unsafe {
                            ports::outw(0x604, 0x2000);
                        }
                    },
                    _ => ()
                }
            }
            _ => ()
        }

        scancode
    }

    fn init(&self) {
        self.terminal.disable_cursor();
        self.terminal.clear();
        splash(self.terminal);

        self.terminal.println("");

        match self.sel {
            0 => {
                self.active_sel("Shell");
                self.inactive_sel("Shut Down");
            },
            1 => {
                self.inactive_sel("Shell");
                self.active_sel("Shut Down");
            },
            _ => ()
        }
    }

    fn active_sel(&self, msg: &str) {
        self.terminal.print("                              ");

        self.terminal.setcolor(VGAColor::Black, VGAColor::White);
        self.terminal.print("    ");
        self.terminal.print(msg);
        for i in 0..(12 - msg.len()) {
            self.terminal.print(" ");
        }
        self.terminal.println("");

        self.terminal.setcolor(VGAColor::White, VGAColor::Black);
    }

    fn inactive_sel(&self, msg: &str) {
        self.terminal.print("                              ");

        self.terminal.setcolor(VGAColor::White, VGAColor::Black);
        self.terminal.print("    ");
        self.terminal.print(msg);
        for i in 0..(12 - msg.len()) {
            self.terminal.print(" ");
        }
        self.terminal.println("");
    }
}

pub fn splash(terminal: &Terminal) {
    terminal.setcolor(VGAColor::Magenta, VGAColor::Black);
    terminal.println("                                   _
                                  | |
              _ __   _   _   ___  | |_   ______    ___    ___
             | '__| | | | | / __| | __| |______|  / _ \\  / __|
             | |    | |_| | \\__ \\ | |_           | (_) | \\__ \\
             |_|     \\__,_| |___/  \\__|           \\___/  |___/

                                                      ");
    terminal.setcolor(VGAColor::White, VGAColor::Black);
}