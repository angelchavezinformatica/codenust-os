pub mod clear;
pub mod echo;

use crate::programs::clear::clear_program;
use crate::programs::echo::echo_program;

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
];
