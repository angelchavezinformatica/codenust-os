use crate::{println, programs::PROGRAMS};

pub fn help_command(argv: &[&str]) {
    if argv.len() > 3 {
        if argv[2] == "-h" {
            println!("Lists all available commands or shows details for one.");
            return;
        }
        println!("err: Subcommand not found");
        return;
    }

    for program in PROGRAMS {
        println!("{} - {}", program.name, program.description);
    }
}
