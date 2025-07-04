// For testing the VGA buffer -- not planning to use C

#include <stdint.h>
#include <stddef.h>

#define BUFFER_HEIGHT 25
#define BUFFER_WIDTH 80


static size_t cursor_x = 0;
static size_t cursor_y = 0;

static uint16_t *buffer = (uint16_t *)0xC00B8000;


void putchar(char c) {
    buffer[cursor_y * BUFFER_WIDTH + cursor_x] = c | (0x0F << 8);
    cursor_x++;
    if (cursor_x >= BUFFER_WIDTH) {
        cursor_x = 0;
        cursor_y++;
    }
}

void callme() {
    putchar('A');
    putchar('B');
    while(1) {}
}