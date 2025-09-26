pub mod clear;
pub mod echo;
pub mod reset;

use crate::programs::clear::clear_program;
use crate::programs::echo::echo_program;
use crate::programs::reset::reset_command;

pub struct Program {
    pub name: &'static str,
    pub entry: fn(&[&str]),
}

pub static PROGRAMS: &[Program] = &[
    Program {
        name: "echo",
        entry: echo_program,
    },
    Program {
        name: "clear",
        entry: clear_program,
    },
    Program {
        name: "reset",
        entry: reset_command,
    },
];
