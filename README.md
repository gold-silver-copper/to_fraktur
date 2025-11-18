# Fraktur Text Converter

[![Crates.io](https://img.shields.io/crates/v/to_fraktur.svg)](https://crates.io/crates/to_fraktur)
[![Documentation](https://docs.rs/to_fraktur/badge.svg)](https://docs.rs/to_fraktur)

A minimal Rust library for converting ASCII text to Fraktur (Gothic) Unicode characters.

## Features

- Convert ASCII alphabetic characters to Fraktur script
- Support for both regular and bold Fraktur styles
- Preserves non-alphabetic characters (numbers, punctuation, spaces)
- Zero dependencies
- Fully tested

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
to_fraktur = "0.1.0"
```

### Basic Example

```rust
use to_fraktur::{to_fraktur, to_fraktur_bold};

fn main() {
    let text = "Hello World!";
    
    // Regular Fraktur
    println!("{}", to_fraktur(text));
    // Output: ℌ𝔢𝔩𝔩𝔬 𝔚𝔬𝔯𝔩𝔡!
    
    // Bold Fraktur
    println!("{}", to_fraktur_bold(text));
    // Output: 𝕳𝖊𝖑𝖑𝖔 𝖂𝖔𝖗𝖑𝖉!
}
```

## API

### `to_fraktur(text: &str) -> String`

Converts ASCII alphabetic characters to regular Fraktur Unicode characters.

- Uppercase: A-Z → 𝔄-ℨ
- Lowercase: a-z → 𝔞-𝔷
- Non-alphabetic characters remain unchanged

### `to_fraktur_bold(text: &str) -> String`

Converts ASCII alphabetic characters to bold Fraktur Unicode characters.

- Uppercase: A-Z → 𝕬-𝖅
- Lowercase: a-z → 𝖆-𝖟
- Non-alphabetic characters remain unchanged

## Character Mappings

### Regular Fraktur

```
A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
𝔄 𝔅 ℭ 𝔇 𝔈 𝔉 𝔊 ℌ ℑ 𝔍 𝔎 𝔏 𝔐 𝔑 𝔒 𝔓 𝔔 ℜ 𝔖 𝔗 𝔘 𝔙 𝔚 𝔛 𝔜 ℨ

a b c d e f g h i j k l m n o p q r s t u v w x y z
𝔞 𝔟 𝔠 𝔡 𝔢 𝔣 𝔤 𝔥 𝔦 𝔧 𝔨 𝔩 𝔪 𝔫 𝔬 𝔭 𝔮 𝔯 𝔰 𝔱 𝔲 𝔳 𝔴 𝔵 𝔶 𝔷
```

### Bold Fraktur

```
A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
𝕬 𝕭 𝕮 𝕯 𝕰 𝕱 𝕲 𝕳 𝕴 𝕵 𝕶 𝕷 𝕸 𝕹 𝕺 𝕻 𝕼 𝕽 𝕾 𝕿 𝖀 𝖁 𝖂 𝖃 𝖄 𝖅

a b c d e f g h i j k l m n o p q r s t u v w x y z
𝖆 𝖇 𝖈 𝖉 𝖊 𝖋 𝖌 𝖍 𝖎 𝖏 𝖐 𝖑 𝖒 𝖓 𝖔 𝖕 𝖖 𝖗 𝖘 𝖙 𝖚 𝖛 𝖜 𝖝 𝖞 𝖟
```

## Testing

Run the test suite:

```bash
cargo test
```

## License

This project is available under your choice of license.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
