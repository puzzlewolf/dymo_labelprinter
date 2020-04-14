/// represents an image that can be printed on a dymo label printer.
pub struct PrintableImage {
    // TODO private
    pub data: Vec<[u8; 8]>,
}

impl PrintableImage {
    pub fn preview(&self) -> String {
        let mut output = String::new();
        self.data.iter().for_each(|row| {
            row
                .iter()
                .for_each(|byte| output.push_str(&format!("{:08b}", byte)));
            output.push_str("\n");
        });
        output.replace("0", ".").replace("1", "X")
    }
}
