// Each signal consists of either a DIT (.), DAH (-) or SPACE.  We can store this in two bits.
pub const DIT: u8 = 0b01;
pub const DAH: u8 = 0b10;
pub const SPC: u8 = 0b00;

///
/// The buffer used for storing morse code messages.
/// This imposes a 128 signal limit on all messages,
/// and each morse letter is made from 1-5 signals.
///
pub type Buffer = [u8; 256];

///
/// Return a signal (DIT, DAH, SPACE) from the buffer at a given index.
///
pub fn get_signal_from_buffer(buffer: &Buffer, index: usize) -> u8 {
    let i = index / 4;
    let j = index % 4;
    (buffer[i] >> (j * 2)) & 0x3
}

///
/// Converts a string into a byte array, where every DIT, DAH or SPACE is stored as two bits.
///
pub fn convert_message_into_morse_buffer(buffer: &mut Buffer, index: &mut usize, s: &str) {
    for c in s.chars() {
        let l = convert_character_to_morse(c);
        append_buffer_letter(buffer, index, l);
    }
}

///
/// Appends a 2-bit signal to the buffer and increments the index
///
fn append_buffer_character(buffer: &mut Buffer, index: &mut usize, c: u8) {
    let i = *index / 4;
    let j = *index % 4;
    buffer[i] |= c << (j * 2);
    *index += 1;
}

///
/// Appends a morse code letter consisting of 1-5 signals
///
fn append_buffer_letter(buffer: &mut Buffer, index: &mut usize, s: &str) {
    for c in s.chars() {
        match c {
            '.' => append_buffer_character(buffer, index, DIT),
            '-' => append_buffer_character(buffer, index, DAH),
            _ => append_buffer_character(buffer, index, SPC),
        }
    }
    append_buffer_character(buffer, index, SPC);
}

///
/// Converts an ASCII character into a sequence of morse signals
///
fn convert_character_to_morse(c: char) -> &'static str {
    match c {
        // Letters
        'A' => ".-",
        'B' => "-...",
        'C' => "-.-.",
        'D' => "-..",
        'E' => ".",
        'F' => "-...",
        'G' => "--.",
        'H' => "...",
        'I' => "..",
        'J' => ".---",
        'K' => "-.-",
        'L' => ".-..",
        'M' => "--",
        'N' => "-.",
        'O' => "---",
        'P' => ".--.",
        'Q' => "--.-",
        'R' => ".-.",
        'S' => "...",
        'T' => "-",
        'U' => "..-",
        'V' => "...-",
        'W' => ".--",
        'X' => "-..-",
        'Y' => "-.--",
        'Z' => "--..",
        // Numbers
        '1' => ".----",
        '2' => "..---",
        '3' => "...--",
        '4' => "....-",
        '5' => ".....",
        '6' => "-....",
        '7' => "--...",
        '8' => "---..",
        '9' => "----.",
        '0' => "-----",
        // Everything else is a space
        _ => " ",
    }
}
