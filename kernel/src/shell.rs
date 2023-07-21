use crate::terminal::{Terminal, VGAColor, self};
use crate::ports;
use crate::command::Command;

pub struct Shell<'a> {
    terminal: &'a Terminal,
    command: Command<'a>,
    shiftdown: bool
}

pub enum Status {
    Ok,
    Exit
}

impl<'a> Shell<'a> {
    pub fn new(terminal: &'a Terminal) -> Self {
        Shell {
            terminal,
            command: Command::new(terminal),
            shiftdown: false
        }
    }

    pub fn run(&mut self) {
        self.init();

        unsafe {
            if ports::inb(0x60) == 0x1C {
                while ports::inb(0x60) != 0x9C {}
            }
        }

        let mut prev = 0;
        let mut status = Status::Ok;
        loop {
            prev = self.handle_events(prev, &mut status);

            match status {
                Status::Ok => (),
                Status::Exit => break
            }
        }
    }

    fn handle_events(&mut self, prev: u8, status: &mut Status) -> u8 {
        let scancode = unsafe {
            ports::inb(0x60)
        };

        if scancode == prev {
            return scancode;
        }

        match scancode {
            0x1C => { // Enter
                let (x, y) = self.terminal.get_pos();
                self.terminal.unsetcursor(x, y);

                *status = self.command.run();

                let (x, y) = self.terminal.get_pos();
                if x != 0 {
                    self.terminal.println("");
                }

                self.prompt();
                let (x, y) = self.terminal.get_pos();
                self.terminal.setcursor(x, y);
            },

            0x2A => { // Left shift
                self.shiftdown = true;
            },
            0x36 => { // Right shift
                self.shiftdown = true;
            },
            0xAA => { // Left shift up
                self.shiftdown = false;
            },
            0xB6 => { // Right shift up
                self.shiftdown = false
            },

            0x0E => { // Delete
                self.command.remove();
            },

            0x39 => { // Space
                self.command.add(" ");
            }

            0x02 => { // 1
                match self.shiftdown {
                    true => self.command.add("!"),
                    false => self.command.add("1")
                }
            },
            0x03 => { // 2
                match self.shiftdown {
                    true => self.command.add("@"),
                    false => self.command.add("2")
                }
            },
            0x04 => { // 3
                match self.shiftdown {
                    true => self.command.add("#"),
                    false => self.command.add("3")
                }
            },
            0x05 => { // 4
                match self.shiftdown {
                    true => self.command.add("$"),
                    false => self.command.add("4")
                }
            },
            0x06 => { // 5
                match self.shiftdown {
                    true => self.command.add("%"),
                    false => self.command.add("5")
                }
            },
            0x07 => { // 6
                match self.shiftdown {
                    true => self.command.add("^"),
                    false => self.command.add("6")
                }
            },
            0x08 => { // 7
                match self.shiftdown {
                    true => self.command.add("&"),
                    false => self.command.add("7")
                }
            },
            0x09 => { // 8
                match self.shiftdown {
                    true => self.command.add("*"),
                    false => self.command.add("8")
                }
            },
            0x0A => { // 9
                match self.shiftdown {
                    true => self.command.add("("),
                    false => self.command.add("9")
                }
            },
            0x0B => { // 0
                match self.shiftdown {
                    true => self.command.add(")"),
                    false => self.command.add("0")
                }
            },

            0x1E => { // A
                match self.shiftdown {
                    true => self.command.add("A"),
                    false => self.command.add("a")
                }
            },
            0x30 => { // B
                match self.shiftdown {
                    true => self.command.add("B"),
                    false => self.command.add("b")
                }
            },
            0x2E => { // C
                match self.shiftdown {
                    true => self.command.add("C"),
                    false => self.command.add("c")
                }
            },
            0x20 => { // D
                match self.shiftdown {
                    true => self.command.add("D"),
                    false => self.command.add("d")
                }
            },
            0x12 => { // E
                match self.shiftdown {
                    true => self.command.add("E"),
                    false => self.command.add("e")
                }
            },
            0x21 => { // F
                match self.shiftdown {
                    true => self.command.add("F"),
                    false => self.command.add("f")
                }
            },
            0x22 => { // G
                match self.shiftdown {
                    true => self.command.add("G"),
                    false => self.command.add("g")
                }
            },
            0x23 => { // H
                match self.shiftdown {
                    true => self.command.add("H"),
                    false => self.command.add("h")
                }
            },
            0x17 => { // I
                match self.shiftdown {
                    true => self.command.add("I"),
                    false => self.command.add("i")
                }
            },
            0x24 => { // J
                match self.shiftdown {
                    true => self.command.add("J"),
                    false => self.command.add("j")
                }
            },
            0x25 => { // K
                match self.shiftdown {
                    true => self.command.add("K"),
                    false => self.command.add("k")
                }
            },
            0x26 => { // L
                match self.shiftdown {
                    true => self.command.add("L"),
                    false => self.command.add("l")
                }
            },
            0x32 => { // M
                match self.shiftdown {
                    true => self.command.add("M"),
                    false => self.command.add("m")
                }
            },
            0x31 => { // N
                match self.shiftdown {
                    true => self.command.add("N"),
                    false => self.command.add("n")
                }
            },
            0x18 => { // O
                match self.shiftdown {
                    true => self.command.add("O"),
                    false => self.command.add("o")
                }
            },
            0x19 => { // P
                match self.shiftdown {
                    true => self.command.add("P"),
                    false => self.command.add("p")
                }
            },
            0x10 => { // Q
                match self.shiftdown {
                    true => self.command.add("Q"),
                    false => self.command.add("q")
                }
            },
            0x13 => { // R
                match self.shiftdown {
                    true => self.command.add("R"),
                    false => self.command.add("r")
                }
            },
            0x1F => { // S
                match self.shiftdown {
                    true => self.command.add("S"),
                    false => self.command.add("s")
                }
            },
            0x14 => { // T
                match self.shiftdown {
                    true => self.command.add("T"),
                    false => self.command.add("t")
                }
            },
            0x16 => { // U
                match self.shiftdown {
                    true => self.command.add("U"),
                    false => self.command.add("u")
                }
            },
            0x2F => { // V
                match self.shiftdown {
                    true => self.command.add("V"),
                    false => self.command.add("v")
                }
            },
            0x11 => { // W
                match self.shiftdown {
                    true => self.command.add("W"),
                    false => self.command.add("w")
                }
            },
            0x2D => { // X
                match self.shiftdown {
                    true => self.command.add("X"),
                    false => self.command.add("x")
                }
            },
            0x15 => { // Y
                match self.shiftdown {
                    true => self.command.add("Y"),
                    false => self.command.add("y")
                }
            },
            0x2C => { // Z
                match self.shiftdown {
                    true => self.command.add("Z"),
                    false => self.command.add("z")
                }
            },

            0x1A => { // [
                match self.shiftdown {
                    true => self.command.add("{"),
                    false => self.command.add("[")
                }
            },
            0x1B => { // ]
                match self.shiftdown {
                    true => self.command.add("}"),
                    false => self.command.add("]")
                }
            },

            0x27 => { // ;
                match self.shiftdown {
                    true => self.command.add(":"),
                    false => self.command.add(";")
                }
            },
            0x28 => { // '
                match self.shiftdown {
                    true => self.command.add("\""),
                    false => self.command.add("\'")
                }
            },

            _ => ()
        }

        scancode
    }

    fn init(&self) {
        self.terminal.clear();
        self.prompt();
        self.update_cursor();
    }

    fn prompt(&self) {
        self.terminal.setcolor(VGAColor::LightBlue, VGAColor::Black);
        self.terminal.print("~ ");
        self.terminal.setcolor(VGAColor::Magenta, VGAColor::Black);
        self.terminal.print("> ");

        self.terminal.setcolor(VGAColor::White, VGAColor::Black);
    }

    fn update_cursor(&self) {
        let (x, y) = self.terminal.get_pos();
        self.terminal.setcursor(x, y);
    }
}