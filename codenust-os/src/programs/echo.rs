use crate::{print, println};

pub fn echo_program(argv: &[&str]) {
    if argv.len() > 2 {
        for arg in &argv[2..] {
            print!("{} ", arg);
        }
        println!("");
    } else {
        println!("echo: missing arguments");
    }
}
