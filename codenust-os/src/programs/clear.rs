use crate::drivers::vga::writer::WRITER;

pub fn clear_program(_argv: &[&str]) {
    WRITER.lock().clear_screen();
}
