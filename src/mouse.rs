use crate::{
    irq,
    ports::{inb, outb},
    system::Registers,
};

const PS2_COMMAND_PORT: u16 = 0x64;
const PS2_DATA_PORT: u16 = 0x60;

const SCREEN_WIDTH: usize = 1024;
const SCREEN_HEIGHT: usize = 768;

const MOUSE_WIDTH: usize = 10;
const MOUSE_HEIGHT: usize = 10;

static mut CURSOR_X: i32 = SCREEN_WIDTH as i32 / 2;
static mut CURSOR_Y: i32 = SCREEN_HEIGHT as i32 / 2;

static mut MOUSE_PACKET: [u8; 3] = [0; 3];
static mut PACKET_INDEX: usize = 0;

unsafe fn mouse_write(cmd: u8) {
    outb(PS2_COMMAND_PORT, 0xD4);
    outb(PS2_DATA_PORT, cmd);
    // Wait for ACK
    while inb(PS2_COMMAND_PORT) & 0x01 == 0 {}
    let _ = inb(PS2_DATA_PORT);
}

pub unsafe fn init_mouse() {
    crate::qemu_println!("Enabling mouse...");
    outb(PS2_COMMAND_PORT, 0xA8);
    outb(PS2_COMMAND_PORT, 0x20);
    let status = inb(PS2_DATA_PORT);
    let new_status = status | 0x02 | 0x20;
    outb(PS2_COMMAND_PORT, 0x60);
    outb(PS2_DATA_PORT, new_status);

    crate::qemu_println!("Mouse: Set defaults");
    mouse_write(0xF6); // Set Defaults

    mouse_write(0xF3);
    mouse_write(200);

    mouse_write(0xF3);
    mouse_write(100);

    mouse_write(0xF3);
    mouse_write(80);

    mouse_write(0xF4);

    // Unmask IRQ12
    let mask = inb(0xA1) & !(1 << 4);
    outb(0xA1, mask);

    update_cursor_position(SCREEN_WIDTH as i32 / 2, SCREEN_HEIGHT as i32 / 2);

    irq::install_handler(12, |r| irq12(r));
}

pub unsafe fn irq12(_r: *const Registers) {
    let byte = inb(PS2_DATA_PORT);

    unsafe {
        MOUSE_PACKET[PACKET_INDEX] = byte;
        PACKET_INDEX += 1;

        if PACKET_INDEX == 3 {
            PACKET_INDEX = 0;
            process_mouse_packet(&MOUSE_PACKET);
        }
    }

    // Notify PIC (platform-specific, not shown)
    // crate::pic::notify_end_of_interrupt(12);
}

fn process_mouse_packet(packet: &[u8; 3]) {
    crate::qemu_println!(
        "Packet: [{:02x} {:02x} {:02x}]",
        packet[0],
        packet[1],
        packet[2]
    );
    // Decode packet bytes into movement and button states
    let x_sign = (packet[0] & 0x10) != 0;
    let y_sign = (packet[0] & 0x20) != 0;

    let mut x_move = packet[1] as i8 as i32;
    let mut y_move = packet[2] as i8 as i32;

    // Update cursor position accordingly
    update_cursor_position(x_move, -y_move); // Y is typically inverted
}

fn update_cursor_position(dx: i32, dy: i32) {
    unsafe {
        CURSOR_X = (CURSOR_X + dx).clamp(0, SCREEN_WIDTH as i32 - MOUSE_WIDTH as i32);
        CURSOR_Y = (CURSOR_Y + dy).clamp(0, SCREEN_HEIGHT as i32 - MOUSE_HEIGHT as i32);

        crate::qemu_println!("Cursor: x = {}, y = {}", CURSOR_X, CURSOR_Y);

        draw_cursor_rect(CURSOR_X as usize, CURSOR_Y as usize);
    }
}

fn draw_cursor_rect(x: usize, y: usize) {
    crate::qemu_println!("Drawing cursor at {}, {}", x, y);
    // Draw a filled rectangle at (x, y) on the framebuffer
    // Example: draw a 10x10 red rectangle
    crate::text::WRITER.lock().framebuffer.draw_rectangle(
        x,
        y,
        MOUSE_WIDTH,
        MOUSE_HEIGHT,
        0xFFFFFF,
    );
}
