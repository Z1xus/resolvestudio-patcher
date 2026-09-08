pub struct Signature {
    bytes: Vec<Option<u8>>,
    anchor: (usize, u8),
}

impl Signature {
    pub fn parse(text: &str) -> Result<Self, String> {
        let bytes: Vec<Option<u8>> = text
            .split_whitespace()
            .map(|token| match token {
                "?" | "??" => Ok(None),
                value if value.len() == 2 && value.bytes().all(|byte| byte.is_ascii_hexdigit()) => {
                    u8::from_str_radix(value, 16)
                        .map(Some)
                        .map_err(|_| format!("invalid signature byte: {value}"))
                }
                _ => Err(format!("invalid signature byte: {token}")),
            })
            .collect::<Result<_, _>>()?;
        let anchor = bytes
            .iter()
            .enumerate()
            .find_map(|(offset, byte)| byte.map(|byte| (offset, byte)))
            .ok_or("signature must contain at least one fixed byte")?;
        Ok(Self { bytes, anchor })
    }

    pub fn width(&self) -> usize {
        self.bytes.len()
    }

    pub fn matches(&self, data: &[u8]) -> bool {
        data.len() == self.width()
            && data[self.anchor.0] == self.anchor.1
            && self
                .bytes
                .iter()
                .zip(data)
                .all(|(expected, actual)| expected.is_none_or(|byte| byte == *actual))
    }

    pub fn find<'a>(&'a self, data: &'a [u8]) -> impl Iterator<Item = usize> + 'a {
        data.windows(self.bytes.len())
            .enumerate()
            .filter_map(|(offset, window)| self.matches(window).then_some(offset))
    }
}
