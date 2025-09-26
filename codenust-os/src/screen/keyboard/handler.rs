use core::arch::asm;

use super::scancodes::SCANCODE_MAP;
use crate::arch_x86_64::pic::send_eoi;
use crate::print;
use crate::programs::cli::process_input;
use crate::{
    arch_x86_64::{
        idt::register_handler,
        pic::{inb, outb},
    },
    println,
};

const INPUT_BUFFER_LEN: usize = 128;
static mut INPUT_BUFFER: [u8; 128] = [0; 128];
static mut INPUT_LEN: usize = 0;

#[unsafe(no_mangle)]
extern "C" fn keyboard_interrupt_handler() {
    unsafe {
        let scancode = inb(0x60);

        // Si es liberación de tecla, ignorar
        if scancode & 0x80 != 0 {
            outb(0x20, 0x20); // EOI
            asm!("sti");
            return;
        }

        if scancode == 0x1C {
            // Enter
            println!();

            let input_str = core::str::from_utf8_unchecked(&INPUT_BUFFER[..INPUT_LEN]);
            process_input(input_str); // se pasa el buffer como parámetro
            INPUT_LEN = 0;
        } else if scancode == 0x0E {
            // Backspace
            //if INPUT_LEN > 0 {
            //    INPUT_LEN -= 1;
            //    backspace_on_vga();
            //}
        } else {
            // Tecla normal
            if let Some(ch) = SCANCODE_MAP[scancode as usize] {
                if INPUT_LEN < INPUT_BUFFER_LEN {
                    INPUT_BUFFER[INPUT_LEN] = ch;
                    INPUT_LEN += 1;
                    print!("{}", ch as char);
                }
            }
        }

        send_eoi();
        asm!("sti");
    }
}

/// Inicializa el teclado: registra el handler en la IDT
pub unsafe fn init_keyboard() {
    let handler_fn = keyboard_interrupt_handler as u64;
    // Vector 0x21 (33 decimal) es IRQ1 del teclado.
    unsafe {
        register_handler(0x21, handler_fn as u64, 0x08, 0x8E00);
    }
}
