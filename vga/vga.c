#include "vga.h"

struct Terminal terminal_init(void) {
	struct Terminal terminal = {
		(uint16_t*)VGA_BUFFER,
		0,
		0,
		vga_entry_color(VGA_COLOR_WHITE, VGA_COLOR_BLACK)
	};
	terminal_clear(&terminal);
	return terminal;
}

void terminal_putchar(uint8_t c, struct Terminal* terminal) {
	if(c == '\n') {
		terminal->row++;
		if(terminal->row >= VGA_HEIGHT) {
			terminal_clear(terminal);
		}
		terminal->column = 0;
		return;
	}

	size_t index = terminal->row * VGA_WIDTH + terminal->column;
	terminal->buffer[index] = vga_entry(c, terminal->color);

	terminal->column++;
	if(terminal->column >= VGA_WIDTH) {
		terminal->row++;
		terminal->column = 0;
	}

	if(terminal->row >= VGA_HEIGHT) {
		terminal_clear(terminal);
	}
}

void terminal_clear(struct Terminal* terminal) {
	terminal->column = 0;
	terminal->row = 0;
	for(int i = 0; i < VGA_WIDTH * VGA_HEIGHT - 1; i++) {
		terminal_putchar(' ', terminal);
	}
	terminal->column = 0;
	terminal->row = 0;
}
