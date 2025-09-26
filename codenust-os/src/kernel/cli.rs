use crate::{print, println};

use crate::programs::PROGRAMS;

pub unsafe fn process_input(input_str: &str) {
    // Tokenizar (separar por espacios)
    let mut parts: heapless::Vec<&str, 16> = heapless::Vec::new();
    for part in input_str.split_whitespace() {
        let _ = parts.push(part); // ignorar si se excede
    }

    if parts.is_empty() {
        return;
    }

    // Construir argv con tu formato
    let mut argv: heapless::Vec<&str, 16> = heapless::Vec::new();
    let _ = argv.push(""); // index 0 → ruta vacía
    let _ = argv.extend(parts.iter().copied()); // index 1 → programa, resto → args

    // Buscar el programa
    for prog in PROGRAMS {
        if argv[1] == prog.name {
            (prog.entry)(&argv);
            println!();
            print!("> ");
            return;
        }
    }

    println!("err: Command not found");
    println!();
    print!("> ");
}
