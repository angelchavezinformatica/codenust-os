/// Mapa de scancodes -> ASCII (simplificado)
pub static SCANCODE_MAP: [Option<u8>; 128] = {
    let mut map = [None; 128];

    map[0x29] = Some(b'`');
    map[0x02] = Some(b'1');
    map[0x03] = Some(b'2');
    map[0x04] = Some(b'3');
    map[0x05] = Some(b'4');
    map[0x06] = Some(b'5');
    map[0x07] = Some(b'6');
    map[0x08] = Some(b'7');
    map[0x09] = Some(b'8');
    map[0x0a] = Some(b'9');
    map[0x0b] = Some(b'0');
    map[0x0c] = Some(b'-');
    map[0x0d] = Some(b'=');

    map[0x10] = Some(b'q');
    map[0x11] = Some(b'w');
    map[0x12] = Some(b'e');
    map[0x13] = Some(b'r');
    map[0x14] = Some(b't');
    map[0x15] = Some(b'y');
    map[0x16] = Some(b'u');
    map[0x17] = Some(b'i');
    map[0x18] = Some(b'o');
    map[0x19] = Some(b'p');

    map[0x1a] = Some(b'[');
    map[0x1b] = Some(b']');
    map[0x2b] = Some(b'\\');

    map[0x1E] = Some(b'a');
    map[0x1F] = Some(b's');
    map[0x20] = Some(b'd');
    map[0x21] = Some(b'f');
    map[0x22] = Some(b'g');
    map[0x23] = Some(b'h');
    map[0x24] = Some(b'j');
    map[0x25] = Some(b'k');
    map[0x26] = Some(b'l');

    map[0x27] = Some(b';');
    map[0x28] = Some(b'\'');

    map[0x2C] = Some(b'z');
    map[0x2D] = Some(b'x');
    map[0x2E] = Some(b'c');
    map[0x2F] = Some(b'v');
    map[0x30] = Some(b'b');
    map[0x31] = Some(b'n');
    map[0x32] = Some(b'm');

    map[0x33] = Some(b',');
    map[0x34] = Some(b'.');
    map[0x35] = Some(b'/');

    map[0x39] = Some(b' ');

    map
};

pub static SCANCODE_MAP_SHIFT: [Option<u8>; 128] = {
    let mut map = [None; 128];

    map[0x29] = Some(b'~');
    map[0x02] = Some(b'!');
    map[0x03] = Some(b'@');
    map[0x04] = Some(b'#');
    map[0x05] = Some(b'$');
    map[0x06] = Some(b'%');
    map[0x07] = Some(b'^');
    map[0x08] = Some(b'&');
    map[0x09] = Some(b'*');
    map[0x0a] = Some(b'(');
    map[0x0b] = Some(b')');
    map[0x0c] = Some(b'_');
    map[0x0d] = Some(b'+');

    map[0x10] = Some(b'Q');
    map[0x11] = Some(b'W');
    map[0x12] = Some(b'E');
    map[0x13] = Some(b'R');
    map[0x14] = Some(b'T');
    map[0x15] = Some(b'Y');
    map[0x16] = Some(b'U');
    map[0x17] = Some(b'I');
    map[0x18] = Some(b'O');
    map[0x19] = Some(b'P');

    map[0x1a] = Some(b'{');
    map[0x1b] = Some(b'}');
    map[0x2b] = Some(b'|');

    map[0x1E] = Some(b'A');
    map[0x1F] = Some(b'S');
    map[0x20] = Some(b'D');
    map[0x21] = Some(b'F');
    map[0x22] = Some(b'G');
    map[0x23] = Some(b'H');
    map[0x24] = Some(b'J');
    map[0x25] = Some(b'K');
    map[0x26] = Some(b'L');

    map[0x27] = Some(b':');
    map[0x28] = Some(b'"');

    map[0x2C] = Some(b'Z');
    map[0x2D] = Some(b'X');
    map[0x2E] = Some(b'C');
    map[0x2F] = Some(b'V');
    map[0x30] = Some(b'B');
    map[0x31] = Some(b'N');
    map[0x32] = Some(b'M');

    map[0x33] = Some(b'<');
    map[0x34] = Some(b'>');
    map[0x35] = Some(b'?');

    map[0x39] = Some(b' ');

    map
};
