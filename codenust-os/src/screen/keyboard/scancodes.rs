/// Mapa de scancodes -> ASCII (simplificado)
pub static SCANCODE_MAP: [Option<u8>; 128] = {
    let mut map = [None; 128];
    // Letras
    map[0x1E] = Some(b'a');
    map[0x30] = Some(b'b');
    map[0x2E] = Some(b'c');
    map[0x20] = Some(b'd');
    map[0x12] = Some(b'e');
    map[0x21] = Some(b'f');
    map[0x22] = Some(b'g');
    map[0x23] = Some(b'h');
    map[0x17] = Some(b'i');
    map[0x24] = Some(b'j');
    map[0x25] = Some(b'k');
    map[0x26] = Some(b'l');
    map[0x32] = Some(b'm');
    map[0x31] = Some(b'n');
    map[0x18] = Some(b'o');
    map[0x19] = Some(b'p');
    map[0x10] = Some(b'q');
    map[0x13] = Some(b'r');
    map[0x1F] = Some(b's');
    map[0x14] = Some(b't');
    map[0x16] = Some(b'u');
    map[0x2F] = Some(b'v');
    map[0x11] = Some(b'w');
    map[0x2D] = Some(b'x');
    map[0x15] = Some(b'y');
    map[0x2C] = Some(b'z');

    // Números
    map[0x02] = Some(b'1');
    map[0x03] = Some(b'2');
    map[0x04] = Some(b'3');
    map[0x05] = Some(b'4');
    map[0x06] = Some(b'5');
    map[0x07] = Some(b'6');
    map[0x08] = Some(b'7');
    map[0x09] = Some(b'8');
    map[0x0A] = Some(b'9');
    map[0x0B] = Some(b'0');

    // Espacio
    map[0x39] = Some(b' ');

    map
};
