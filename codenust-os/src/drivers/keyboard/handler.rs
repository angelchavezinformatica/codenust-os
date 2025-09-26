use super::scancodes::{SCANCODE_MAP, SCANCODE_MAP_SHIFT};
use crate::arch::x86_64::pic::send_eoi;
use crate::drivers::vga::writer::WRITER;
use crate::kernel::cli::process_input;
use crate::print;
use crate::{
    arch::x86_64::{idt::register_handler, pic::inb},
    println,
};

const INPUT_BUFFER_LEN: usize = 128;
static mut INPUT_BUFFER: [u8; 128] = [0; 128];
static mut INPUT_LEN: usize = 0;

// mejor ponerlos a nivel de módulo (fuera de la función)
static mut SHIFT_DOWN: bool = false;
static mut CTRL_DOWN: bool = false;
static mut E0_PREFIX: bool = false;

#[unsafe(no_mangle)]
extern "C" fn keyboard_interrupt_handler() {
    unsafe {
        let scancode = inb(0x60);

        // Prefijo extendido: marca y termina (pero no olvides EOI)
        if scancode == 0xE0 {
            E0_PREFIX = true;
            send_eoi(); // avisar al PIC
            core::arch::asm!("sti");
            return;
        }

        // Shift pressed / released (hacemos EOI antes de salir)
        match scancode {
            0x2A | 0x36 => {
                // make (Shift down)
                SHIFT_DOWN = true;
                send_eoi();
                core::arch::asm!("sti");
                return;
            }
            0xAA | 0xB6 => {
                // break (Shift up)
                SHIFT_DOWN = false;
                send_eoi();
                core::arch::asm!("sti");
                return;
            }
            _ => {}
        }

        // Manejo de Ctrl (soporta Right Ctrl con prefijo E0)
        match (E0_PREFIX, scancode) {
            (false, 0x1D) => CTRL_DOWN = true,  // Left Ctrl make
            (false, 0x9D) => CTRL_DOWN = false, // Left Ctrl break
            (true, 0x1D) => CTRL_DOWN = true,   // Right Ctrl make (E0 1D)
            (true, 0x9D) => CTRL_DOWN = false,  // Right Ctrl break (E0 9D)
            _ => {}
        }

        // ya consumimos el prefijo
        E0_PREFIX = false;

        // Si es liberación genérica (make | 0x80) ignoramos (pero avisamos EOI)
        if scancode & 0x80 != 0 {
            send_eoi();
            core::arch::asm!("sti");
            return;
        }

        // Enter
        if scancode == 0x1C {
            println!();
            let input_str = core::str::from_utf8_unchecked(&INPUT_BUFFER[..INPUT_LEN]);
            process_input(input_str);
            INPUT_LEN = 0;
            send_eoi();
            core::arch::asm!("sti");
            return;
        }

        // Backspace
        if scancode == 0x0E {
            if INPUT_LEN > 0 {
                INPUT_LEN -= 1;
                INPUT_BUFFER[INPUT_LEN] = 0;
                WRITER.lock().backspace();
            }
            send_eoi();
            core::arch::asm!("sti");
            return;
        }

        // Tecla normal: escoger tabla según SHIFT_DOWN
        let ch_opt = if SHIFT_DOWN {
            SCANCODE_MAP_SHIFT[scancode as usize]
        } else {
            SCANCODE_MAP[scancode as usize]
        };

        if let Some(ch) = ch_opt {
            if INPUT_LEN < INPUT_BUFFER_LEN {
                INPUT_BUFFER[INPUT_LEN] = ch;
                INPUT_LEN += 1;
                print!("{}", ch as char);
            }
        }

        // FIN: avisar EOI y reactivar interrupciones
        send_eoi();
        core::arch::asm!("sti");
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
