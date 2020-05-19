use std::io::{Error, ErrorKind};
use std::str;

pub fn get_fonts() -> Result< Vec<String>, Box<dyn std::error::Error>> {
    use std::process::Command;
    let output = Command::new(crate::IM_CONVERT)
        .args(&["-list", "font"])
        .output()
        .expect("failed to execute imagemagick");
    if output.status.success() {
        let prefix = "  Font: ";
        let font_names = str::from_utf8(&output.stdout)?
            .lines()
            .filter(|s| s.starts_with(prefix))
            .map(|s| s.split_at(prefix.len()).1)
            .map(|s| s.into())
            .collect::<Vec<String>>();
        Ok(font_names)
    } else {
        error!("{}", String::from_utf8_lossy(&output.stderr));
        let errortext = "Error during imagemagick fonts query.";
        Err(Error::new(ErrorKind::Other, errortext).into())
    }
}

#[test]
fn test_get_fonts() {
    let foo = get_fonts().unwrap();
    assert!(!foo.is_empty());
}
