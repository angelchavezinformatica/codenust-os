use crate::println;

use super::clear::clear_program;

pub fn print_banner() {
    println!("   _____          _                      _    ____   _____ ");
    println!("  / ____|        | |                    | |  / __ \\ / ____|");
    println!(" | |     ___   __| | ___ _ __  _   _ ___| |_| |  | | (___  ");
    println!(" | |    / _ \\ / _` |/ _ \\ '_ \\| | | / __| __| |  | |\\___ \\ ");
    println!(" | |___| (_) | (_| |  __/ | | | |_| \\__ \\ |_| |__| |____) |");
    println!("  \\_____\\___/ \\__,_|\\___|_| |_|\\__,_|___/\\__|\\____/|_____/ ");
    println!();
}

pub fn reset_command(argv: &[&str]) {
    clear_program(argv);

    print_banner();
}
