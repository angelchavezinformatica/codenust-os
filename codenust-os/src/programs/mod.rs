pub mod clear;
pub mod echo;
pub mod help;
pub mod mailbox;
pub mod reset;

use crate::programs::clear::clear_program;
use crate::programs::echo::echo_program;
use crate::programs::help::help_command;
use crate::programs::mailbox::mailbox_program;
use crate::programs::reset::reset_command;

pub struct Program {
    pub name: &'static str,
    pub entry: fn(&[&str]),
    pub description: &'static str,
}

pub static PROGRAMS: &[Program] = &[
    Program {
        name: "echo",
        entry: echo_program,
        description: "Prints the given arguments to the screen.",
    },
    Program {
        name: "clear",
        entry: clear_program,
        description: "Clears the entire screen.",
    },
    Program {
        name: "help",
        entry: help_command,
        description: "Lists all available commands or shows details for one.",
    },
    Program {
        name: "mb",
        entry: mailbox_program,
        description: "IPC message passing with named mailboxes",
    },
    Program {
        name: "reset",
        entry: reset_command,
        description: "Reboots the system.",
    },
];
