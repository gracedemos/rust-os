use crate::terminal::{Terminal, VGAColor, self};
use crate::ports;
use crate::shell::{Shell, Status};
use crate::menu;

pub struct Command<'a> {
    terminal: &'a Terminal,
    buffer: [u8; terminal::VGA_WIDTH * terminal::VGA_HEIGHT],
    index: usize
}

impl<'a> Command<'a> {
    pub fn new(terminal: &'a Terminal) -> Self {
        Command {
            terminal,
            buffer: [0; terminal::VGA_WIDTH * terminal::VGA_HEIGHT],
            index: 0
        }
    }

    pub fn run(&mut self) -> Status {
        let mut status = Status::Ok;

        if self.buffer[..4] == "exit".as_bytes()[..] && self.index == 4 {
            status = Status::Exit;
        }
        if self.buffer[..6] == "splash".as_bytes()[..] && self.index == 6 {
            self.terminal.println("");
            menu::splash(self.terminal);
        }
        if self.buffer[..5] == "clear".as_bytes()[..] && self.index == 5 {
            self.terminal.clear();
        }
        if self.buffer[..5] == "echo ".as_bytes()[..] {
            self.terminal.println("");
            let msg = &self.buffer[5..self.index];
            self.terminal.print(msg);
        }
        if self.buffer[0..4] == "help".as_bytes()[..] {
            self.terminal.println("");

            self.terminal.println("help: Displays commands");
            self.terminal.println("splash: Displays splash ascii art");
            self.terminal.println("echo <message>: Prints a message to the terminal");
            self.terminal.println("clear: Clears the terminal");
            self.terminal.println("exit: Exits the terminal session");
        }

        self.buffer = [0; terminal::VGA_WIDTH * terminal::VGA_HEIGHT];
        self.index = 0;

        status
    }

    pub fn add(&mut self, c: &str) {
        if c.len() != 1 {
            return;
        }

        self.buffer[self.index] = c.chars()
            .next()
            .unwrap() as u8;
        self.index += 1;

        let (x, y) = self.terminal.get_pos();
        self.terminal.unsetcursor(x, y);

        self.terminal.print(c);

        let (x, y) = self.terminal.get_pos();
        self.terminal.setcursor(x, y);
    }

    pub fn remove(&mut self) {
        if self.index == 0 {
            return;
        }

        self.buffer[self.index] = ' ' as u8;
        self.index -= 1;

        let (x, y) = self.terminal.get_pos();
        self.terminal.unsetcursor(x, y);

        self.terminal.delete();

        let (x, y) = self.terminal.get_pos();
        self.terminal.setcursor(x, y);
    }
}