# ImagHex

ImagHex is a simple tool for encoding files into graphical representations of their bytes and vice versa.  
The graphical representation is a PNG image where each pixel represents 4 bytes of the file.

## Features

- Encode files into PNG images
- Decode PNG images back into files
- Support all input file types (the output is always a PNG image) and sizes (limited by available memory)

## Usage

Encode a file :
```shell
imaghex encode <input_file> <output_dir>
```

Decode a file :
```shell
imaghex decode <input_file.png> <output_dir>
```

## Example

Input file `secret.mp4` :  
🎵🕺

Encoded image `secret.png` :

![example.png](https://github.com/user-attachments/assets/be19ffa5-ce93-47f7-a72b-14b11f40136a)

## Build

### Requirements

- Rust
- Cargo
- Git (optional)

Clone the repository :
```shell
git clone https://github.com/Nathan-Etave/ImagHex.git
cd ImagHex
```
or download the source code from the [latest release](?) and extract it.

Build the project :
```shell
cargo build --release
```

The binary is located in the `target/release` directory.

## License

[MIT](https://choosealicense.com/licenses/mit/)

## Authors

- [Nathan Etave](https://github.com/Nathan-Etave)