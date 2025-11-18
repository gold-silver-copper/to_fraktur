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

pub fn to_fraktur_bold(text: &str) -> String {
    const UPPER: [char; 26] = [
        '𝕬', '𝕭', '𝕮', '𝕯', '𝕰', '𝕱', '𝕲', '𝕳', '𝕴', '𝕵', '𝕶', '𝕷', '𝕸', '𝕹', '𝕺', '𝕻', '𝕼', '𝕽',
        '𝕾', '𝕿', '𝖀', '𝖁', '𝖂', '𝖃', '𝖄', '𝖅',
    ];

    const LOWER: [char; 26] = [
        '𝖆', '𝖇', '𝖈', '𝖉', '𝖊', '𝖋', '𝖌', '𝖍', '𝖎', '𝖏', '𝖐', '𝖑', '𝖒', '𝖓', '𝖔', '𝖕', '𝖖', '𝖗',
        '𝖘', '𝖙', '𝖚', '𝖛', '𝖜', '𝖝', '𝖞', '𝖟',
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

    #[test]
    fn test_bold_uppercase() {
        assert_eq!(to_fraktur_bold("HELLO"), "𝕳𝕰𝕷𝕷𝕺");
    }

    #[test]
    fn test_bold_lowercase() {
        assert_eq!(to_fraktur_bold("world"), "𝖜𝖔𝖗𝖑𝖉");
    }

    #[test]
    fn test_bold_mixed() {
        assert_eq!(to_fraktur_bold("Hello World!"), "𝕳𝖊𝖑𝖑𝖔 𝖂𝖔𝖗𝖑𝖉!");
    }
}
