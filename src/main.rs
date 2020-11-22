#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            // Attribute 	Character
            // 7 	6 	5 	4 	3 	2 	1 	0 	7 	6 	5 	4 	3 	2 	1 	0
            // Blink[n 1] 	Background color 	Foreground color 	Code point 
            *vga_buffer.offset(i as isize * 2 + 1) = 0x9;
        }
    }

    loop {}
}
