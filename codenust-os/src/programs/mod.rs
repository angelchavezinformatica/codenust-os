use crate::screen::vga::writer::WRITER;
use crate::{print, println};
use heapless;

pub struct Program {
    pub name: &'static str,
    pub entry: fn(&[&str]),
}

fn echo_program(argv: &[&str]) {
    if argv.len() > 2 {
        for arg in &argv[2..] {
            print!("{} ", arg);
        }
        println!("");
    } else {
        println!("echo: missing arguments");
    }
}

fn clear_program(_argv: &[&str]) {
    WRITER.lock().clear_screen();
}

static PROGRAMS: &[Program] = &[
    Program {
        name: "echo",
        entry: echo_program,
    },
    Program {
        name: "clear",
        entry: clear_program,
    },
];

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
