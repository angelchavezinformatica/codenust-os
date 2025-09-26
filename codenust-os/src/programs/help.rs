use crate::{println, programs::PROGRAMS};

pub fn help_command(argv: &[&str]) {
    match argv.len() {
        // defensivo: si por alguna razón argv viene vacío
        0 | 1 => {
            println!("Usage: help [COMMAND]");
            println!("Lists all available commands or shows details for a specific command.");
            return;
        }

        // "help" -> listar comandos
        2 => {
            println!("Available commands:");
            for program in PROGRAMS {
                // imprime: name - description
                println!("  {:<8} - {}", program.name, program.description);
            }
            return;
        }

        // "help X" -> X puede ser -h/--help o el nombre de un comando
        3 => {
            let arg = argv[2];
            if arg == "-h" || arg == "--help" {
                println!("Usage: help [COMMAND]");
                println!("Without arguments: list all commands.");
                println!("With a command name: show detailed help for that command.");
                return;
            }

            // buscar el comando en la tabla
            for program in PROGRAMS {
                if program.name == arg {
                    // mostrar descripción general
                    println!("{} - {}", program.name, program.description);

                    // ayuda detallada por comando (extiéndelo según necesites)
                    match program.name {
                        "echo" => {
                            println!("Usage: echo TEXT");
                            println!("Prints TEXT to the screen. Example: echo hello world");
                        }
                        "clear" => {
                            println!("Usage: clear");
                            println!("Clears the entire screen.");
                        }
                        "help" => {
                            println!("Usage: help [COMMAND]");
                            println!("Show a list of commands or details for COMMAND.");
                        }
                        "reset" => {
                            println!("Usage: reset");
                            println!("Reboots the system.");
                        }
                        _ => {
                            // Si no hay detalle extra, no pasa nada
                        }
                    }
                    return;
                }
            }

            // no se encontró el comando
            println!("err: no help for '{}'", arg);
            return;
        }

        // demasiados argumentos
        _ => {
            println!("err: too many arguments for help");
            return;
        }
    }
}
