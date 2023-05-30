#ifndef VGA_H
#define VGA_H

#include <stdint.h>
#include <stddef.h>

#define VGA_WIDTH 80
#define VGA_HEIGHT 25
#define VGA_BUFFER 0xB8000

enum VGAColor {
	VGA_COLOR_BLACK = 0,
	VGA_COLOR_BLUE = 1,
	VGA_COLOR_GREEN = 2,
	VGA_COLOR_CYAN = 3,
	VGA_COLOR_RED = 4,
	VGA_COLOR_MAGENTA = 5,
	VGA_COLOR_BROWN = 6,
	VGA_COLOR_LIGHT_GREY = 7,
	VGA_COLOR_DARK_GREY = 8,
	VGA_COLOR_LIGHT_BLUE = 9,
	VGA_COLOR_LIGHT_GREEN = 10,
	VGA_COLOR_LIGHT_CYAN = 11,
	VGA_COLOR_LIGHT_RED = 12,
	VGA_COLOR_LIGHT_MAGENTA = 13,
	VGA_COLOR_LIGHT_BROWN = 14,
	VGA_COLOR_WHITE = 15,
};

struct Terminal {
	uint16_t* buffer;
	size_t column;
	size_t row;
	uint8_t color;
};

static inline uint8_t vga_entry_color(enum VGAColor fg, enum VGAColor bg) {
	return fg | bg << 4;
}

static inline uint16_t vga_entry(uint8_t c, uint8_t color) {
	return (uint16_t)c | (uint16_t)color << 8;
}

struct Terminal terminal_init(void);
void terminal_putchar(uint8_t c, struct Terminal* terminal);
void terminal_clear(struct Terminal* terminal);

#endif
