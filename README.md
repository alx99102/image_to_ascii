# Image and Video to ASCII Converter

`image_to_ascii` is a command line utility written in Rust that allows you to convert image or video files into their ASCII art representation and display it in the terminal.

## Features

- Supports both image and video files for conversion to ASCII art.
- Takes an image or video file as input and displays the ASCII representation directly to the terminal.
- Resizes the image or video frames to fit the current terminal dimensions.
- Uses luminance to determine the correct ASCII character for each pixel.
- Colors each ASCII character according to the original pixel color.
- For videos, extracts frames using `ffmpeg` and then converts each frame to ASCII representation.

## Requirements

- Rust programming language
- Cargo, the Rust package manager
- `ffmpeg` for video processing

## Dependencies

- `termsize`: To get the terminal dimensions.
- `image`: For image reading and decoding.
- `ansi_term`: To paint ASCII characters with color in the terminal.
- `crossterm`: For cursor manipulation in terminal.

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

   - For image:
     ```bash
     ./target/release/image_to_ascii image path/to/your/image.jpg
     ```

   - For video:
     ```bash
     ./target/release/image_to_ascii video path/to/your/video.mp4
     ```

You should see the ASCII representation of the provided image or video printed in the terminal.

## Notes

- Ensure your terminal supports ANSI colors to view the colored ASCII output.
- The image or video will be resized to fit your terminal dimensions. Ensure your terminal is at a reasonable size for best representation.
- Works best with media that have distinct color contrasts.
- Ensure `ffmpeg` is installed and added to your PATH variable when processing videos.