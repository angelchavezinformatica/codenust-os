#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod arch_x86_64;
mod programs;
mod screen;

use arch_x86_64::{
    idt::init_idt,
    pic::{enable_keyboard_irq, remap_pic},
};

use crate::screen::keyboard::handler::init_keyboard;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn print_banner() {
    println!("   _____          _                      _    ____   _____ ");
    println!("  / ____|        | |                    | |  / __ \\ / ____|");
    println!(" | |     ___   __| | ___ _ __  _   _ ___| |_| |  | | (___  ");
    println!(" | |    / _ \\ / _` |/ _ \\ '_ \\| | | / __| __| |  | |\\___ \\ ");
    println!(" | |___| (_) | (_| |  __/ | | | |_| \\__ \\ |_| |__| |____) |");
    println!("  \\_____\\___/ \\__,_|\\___|_| |_|\\__,_|___/\\__|\\____/|_____/ ");
    println!();
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    unsafe {
        // 1) Remapeamos el PIC
        remap_pic();

        // 2) Inicializamos la IDT
        init_idt();

        // 3) Inicializamos el teclado (registra su ISR)
        init_keyboard();

        // 4) Habilitamos IRQ1 (teclado)
        enable_keyboard_irq();

        // 5) Activamos interrupciones
        core::arch::asm!("sti");
    }

    // 6) Probamos la salida por VGA
    print_banner();
    print!("> ");

    // Bucle infinito
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
