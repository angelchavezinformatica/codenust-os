pub mod structures;

use crate::{
    println,
    programs::mailbox::structures::{MAILBOXES, Mailbox, Message},
};

pub fn mailbox_program(argv: &[&str]) {
    if argv.len() < 3 {
        println!("Usage: mb <create|send|read|list|status> ...");
        return;
    }

    match argv[2] {
        "create" => {
            if argv.len() < 4 {
                println!("Usage: mb create <name>");
                return;
            }

            let mut name_buf = heapless::String::<32>::new();
            let _ = name_buf.push_str(argv[3]); // copia el nombre al buffer estático

            let mut mboxes = MAILBOXES.lock();
            if mboxes.iter().any(|mb| mb.name == name_buf) {
                println!("err: Mailbox '{}' already exists", argv[3]);
                return;
            }

            if mboxes
                .push(Mailbox {
                    name: name_buf,
                    messages: heapless::Vec::new(),
                })
                .is_err()
            {
                println!("err: too many mailboxes");
                return;
            }

            println!("Mailbox '{}' created", argv[3]);
        }

        "send" => {
            if argv.len() < 6 {
                println!("Usage: mb send <from> <to> <message>");
                return;
            }
            let from_str = argv[3];
            let to_str = argv[4];

            // Copiar `from` a un String<32>
            let mut from_buf = heapless::String::<32>::new();
            let _ = from_buf.push_str(from_str);

            // Construir el mensaje concatenando argv[5..]
            let mut message = heapless::String::<128>::new();
            for (i, part) in argv[5..].iter().enumerate() {
                if i > 0 {
                    let _ = message.push(' ');
                }
                let _ = message.push_str(part);
            }

            let mut mboxes = MAILBOXES.lock();
            if let Some(mb) = mboxes.iter_mut().find(|mb| mb.name == to_str) {
                if mb
                    .messages
                    .push(Message {
                        from: from_buf,
                        text: message,
                    })
                    .is_err()
                {
                    println!("err: mailbox '{}' is full", to_str);
                    return;
                }
                println!("Message sent to '{}'", to_str);
            } else {
                println!("err: Mailbox '{}' not found", to_str);
            }
        }

        "read" => {
            if argv.len() < 4 {
                println!("Usage: mb read <name>");
                return;
            }
            let name = argv[3];
            let mut mboxes = MAILBOXES.lock();
            if let Some(mb) = mboxes.iter_mut().find(|mb| mb.name == name) {
                println!("Mailbox '{}':", name);
                for (i, msg) in mb.messages.iter().enumerate() {
                    println!("[{}] {}: {}", i, msg.from, msg.text);
                }
            } else {
                println!("err: Mailbox '{}' not found", name);
            }
        }

        "list" => {
            let mboxes = MAILBOXES.lock();
            println!("Active mailboxes:");
            for mb in mboxes.iter() {
                println!("- {} ({} messages)", mb.name, mb.messages.len());
            }
        }

        "status" => {
            if argv.len() < 4 {
                println!("Usage: mb status <name>");
                return;
            }
            let name = argv[3];
            let mboxes = MAILBOXES.lock();
            if let Some(mb) = mboxes.iter().find(|mb| mb.name == name) {
                let used_bytes: usize = mb.messages.iter().map(|m| m.text.len()).sum();
                println!(
                    "Mailbox '{}': {} messages, {} bytes used",
                    name,
                    mb.messages.len(),
                    used_bytes
                );
            } else {
                println!("err: Mailbox '{}' not found", name);
            }
        }

        _ => {
            println!("err: unknown subcommand '{}'", argv[2]);
        }
    }
}
