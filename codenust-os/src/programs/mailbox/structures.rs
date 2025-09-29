use heapless::Vec;
use spin::Mutex;

// Constantes de límite (por ahora fijas)
pub const MAX_MAILBOXES: usize = 8;
pub const MAX_MESSAGES: usize = 16;

#[derive(Clone)]
pub struct Message {
    pub from: heapless::String<32>,
    pub text: heapless::String<128>,
    // en un futuro podrías poner timestamp real
}

#[derive(Clone)]
pub struct Mailbox {
    pub name: heapless::String<32>,
    pub messages: Vec<Message, MAX_MESSAGES>,
}

lazy_static::lazy_static! {
    pub static ref MAILBOXES: Mutex<Vec<Mailbox, MAX_MAILBOXES>> = Mutex::new(Vec::new());
}
