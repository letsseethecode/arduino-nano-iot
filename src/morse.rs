pub const DIT: u8 = 0x01;
pub const DAH: u8 = 0x02;
pub const SPC: u8 = 0x00;

type Buffer = [u8; 256];

pub fn get_signal(buffer: &Buffer, index: usize) -> u8 {
    let i = index / 4;
    let j = index % 4;
    (buffer[i] >> (j * 2)) & 0x3
}

pub fn append_buffer(buffer: &mut Buffer, index: &mut usize, s: &str) {
    for c in s.chars() {
        let l = convert_character_to_morse(c);
        append_buffer_letter(buffer, index, l);
    }
}

fn append_buffer_character(buffer: &mut Buffer, index: &mut usize, c: u8) {
    let i = *index / 4;
    let j = *index % 4;
    buffer[i] |= c << (j * 2);
    *index += 1;
}

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
