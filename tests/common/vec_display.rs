// Auxiliary struct for displaying vector contents
pub struct VecDisplay<'a>(&'a [u8]);

impl<'a> VecDisplay<'a> {
    pub fn new(vec: &'a [u8]) -> VecDisplay<'a> {
        VecDisplay(vec)
    }
}

impl std::fmt::Display for VecDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (utf8_symbols, hex_symbols): (Vec<_>, Vec<_>) = self.0.iter().map(|b| {(
            String::from_utf8_lossy(&[*b]).into_owned(),
            format!("0x{:02X}", b),
        )}).unzip();
        write!(f, "{:?} ({:?})", utf8_symbols.join(""), hex_symbols)
    }
}