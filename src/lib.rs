pub fn to_fraktur(text: &str) -> String {
    const UPPER: [char; 26] = [
        '𝔄', '𝔅', 'ℭ', '𝔇', '𝔈', '𝔉', '𝔊', 'ℌ', 'ℑ', '𝔍', '𝔎', '𝔏', '𝔐', '𝔑', '𝔒', '𝔓', '𝔔', 'ℜ',
        '𝔖', '𝔗', '𝔘', '𝔙', '𝔚', '𝔛', '𝔜', 'ℨ',
    ];

    const LOWER: [char; 26] = [
        '𝔞', '𝔟', '𝔠', '𝔡', '𝔢', '𝔣', '𝔤', '𝔥', '𝔦', '𝔧', '𝔨', '𝔩', '𝔪', '𝔫', '𝔬', '𝔭', '𝔮', '𝔯',
        '𝔰', '𝔱', '𝔲', '𝔳', '𝔴', '𝔵', '𝔶', '𝔷',
    ];

    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                if c.is_ascii_uppercase() {
                    UPPER[(c as u8 - b'A') as usize]
                } else {
                    LOWER[(c as u8 - b'a') as usize]
                }
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uppercase() {
        assert_eq!(to_fraktur("HELLO"), "ℌ𝔈𝔏𝔏𝔒");
    }

    #[test]
    fn test_lowercase() {
        assert_eq!(to_fraktur("world"), "𝔴𝔬𝔯𝔩𝔡");
    }

    #[test]
    fn test_mixed() {
        assert_eq!(to_fraktur("Hello World!"), "ℌ𝔢𝔩𝔩𝔬 𝔚𝔬𝔯𝔩𝔡!");
    }

    #[test]
    fn test_non_alphabetic() {
        assert_eq!(to_fraktur("123 !@#"), "123 !@#");
    }
}
