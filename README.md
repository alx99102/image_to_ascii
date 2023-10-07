# Image to ASCII Converter

`image_to_ascii` is a command line utility written in Rust that allows you to convert image files into their ASCII art representation and print it to the console.

## Features

- Takes image file as input and displays the ASCII representation directly to the terminal.
- Resizes the image to fit the current terminal dimensions.
- Uses luminance to determine the correct ASCII character for each pixel.
- Colors each ASCII character according to the original pixel color.

## Requirements

- Rust programming language
- Cargo, the Rust package manager

## Dependencies

- `termsize`: To get the terminal dimensions.
- `image`: For image reading and decoding.
- `ansi_term`: To paint ASCII characters with color in the terminal.

## Usage

1. Clone the repository:
    ```bash
    git clone https://github.com/alx99102/image_to_ascii.git
    cd image_to_ascii
    ```

2. Build the project using Cargo:
    ```bash
    cargo build --release
    ```

3. Run the utility:
    ```bash
    ./target/release/image_to_ascii path/to/your/image.jpg
    ```

You should see the ASCII representation of the provided image printed in the terminal.

## Notes

- Ensure your terminal supports ANSI colors to view the colored ASCII output.
- The image will be resized to fit your terminal dimensions. Ensure your terminal is at a reasonable size for best representation.
- Works best with images that have distinct color contrasts.
