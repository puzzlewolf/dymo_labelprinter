use crate::picture::PrintableImage;

#[derive(Default)]
pub struct CommandAccumulator {
    pub accu: Vec<u8>,
}

impl CommandAccumulator {
    const SYN: u8 = 0x16;
    const ESC: u8 = 0x1b;

    // constants:
    // SYN = 0x16 //marks start of line
    // ESC = 0x1b //next byte encodes command
    //      commands according to imgprint perlscript
    //      A getstatus
    //      B dottab, one argument, 0 known used
    //      C tapecolour, one argument, 0 known used
    //      D bytesperline, one argument, used as ESC, D, num_of_bytes e.g. 1b 44 07

    pub fn new() -> Self {
        CommandAccumulator {
            accu: Default::default(),
        }
    }

    fn append_image_rows(&mut self, image: &PrintableImage) {
        self.bytes_per_line(8);
        image.data.iter().for_each(|row| self.add_data_row(*row));
    }

    /// Generates all commands necessary to print an image.
    /// Includes some whitespace in front of and behind the label.
    pub fn generate_commands(&mut self, image: &PrintableImage) {
        self.preamble(false);
        self.append_image_rows(image);
        self.whitespace(56);
        self.postamble(true);
    }

    /// Add the print commands for one row of the image.
    /// Before the line, `bytes_per_line` must be set to the correct value.
    fn add_data_row(&mut self, row: [u8; 8]) {
        self.accu.push(Self::SYN);
        self.accu.extend(&row);
    }

    fn preamble(&mut self, add_whitespace: bool) {
        self.tape_color();
        self.dottab();
        if add_whitespace {
            self.whitespace(56);
        };
    }

    fn postamble(&mut self, add_whitespace: bool) {
        if add_whitespace {
            self.whitespace(56);
        };
        self.get_status();
    }

    /// Add `num` lines of whitespace.
    ///
    /// 56 is recommended as space before and after the label text.
    fn whitespace(&mut self, num: usize) {
        self.bytes_per_line(0);
        (0..num).for_each(|_| self.accu.push(Self::SYN));
    }

    /// The number of bytes in the following row(s).
    /// Seems to take no arguments.
    fn get_status(&mut self) {
        self.accu.push(Self::ESC);
        self.accu.push(b'A');
    }

    /// The number of bytes in the following row(s).
    /// Seems to take one byte argument.
    fn bytes_per_line(&mut self, num: u8) {
        self.accu.push(Self::ESC);
        self.accu.push(b'D');
        self.accu.push(num);
    }

    /// The tape's color. Encoding unknown.
    /// Seems to take one byte argument.
    fn tape_color(&mut self) {
        self.accu.push(Self::ESC);
        self.accu.push(b'C');
        self.accu.push(0 as u8);
    }

    /// Probably whether (or how?) to print the tab character.
    /// Seems to take one byte argument.
    fn dottab(&mut self) {
        self.accu.push(Self::ESC);
        self.accu.push(b'B');
        self.accu.push(0 as u8);
    }
}

#[test]
fn test_append_row() {
    let mut ca = CommandAccumulator { accu: Vec::new() };
    (0..7).for_each(|_| ca.accu.push(17u8));

    ca.add_data_row([0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8]);
    assert_eq!(ca.accu[0..7], [17u8; 7]);
    assert_eq!(ca.accu[7], 0x16);
    assert_eq!(ca.accu[8..16], [0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_preamble() {
    let mut ca = CommandAccumulator { accu: Vec::new() };
    ca.preamble(false);
    assert_eq!(ca.accu[0..6], [0x1b, 0x43, 0, 0x1b, 0x42, 0]);
}

#[test]
fn test_get_status() {
    let mut ca = CommandAccumulator { accu: Vec::new() };
    ca.get_status();
    assert_eq!(ca.accu[0..2], [0x1b, 0x41]);
}

#[test]
fn test_bytes_per_line() {
    let mut ca = CommandAccumulator { accu: Vec::new() };
    ca.bytes_per_line(8);
    assert_eq!(ca.accu[0..3], [0x1b, 0x44, 0x08]);
}

#[test]
fn test_tape_color() {
    let mut ca = CommandAccumulator { accu: Vec::new() };
    ca.tape_color();
    assert_eq!(ca.accu[0..3], [0x1b, 0x43, 0]);
}

#[test]
fn test_dottab() {
    let mut ca = CommandAccumulator { accu: Vec::new() };
    ca.dottab();
    assert_eq!(ca.accu[0..3], [0x1b, 0x42, 0]);
}
