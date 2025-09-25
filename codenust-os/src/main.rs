#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod screen;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("   _____          _                      _    ____   _____");
    println!("  / ____|        | |                    | |  / __ \\ / ____|");
    println!(" | |     ___   __| | ___ _ __  _   _ ___| |_| |  | | (___  ");
    println!(" | |    / _ \\ / _` |/ _ \\ '_ \\| | | / __| __| |  | |\\___ \\ ");
    println!(" | |___| (_) | (_| |  __/ | | | |_| \\__ \\ |_| |__| |____) |");
    println!("  \\_____\\___/ \\__,_|\\___|_| |_|\\__,_|___/\\__|\\____/|_____/ ");

    loop {}
}
