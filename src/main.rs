use termsize;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use ansi_term::Colour::RGB;
use std::env;
use crossterm::cursor::MoveTo;
use std::fs;
use std::process::Command;
use crossterm::execute;



fn main() {
    
    // clear tmp dir
    let _ = clear_dir();
    

    // get args
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Please provide a type (image/video) and a file path");
        return;
    }

    if args[1] == "image" {
        print_frame(std::path::Path::new(&args[2]), &termsize::get().unwrap());
        return;
    } else if args[1] != "video" {
        println!("Please provide a valid type (image/video)");
        return;
    }

    // invoke ffmpeg
    let ffmpeg_check = Command::new("ffmpeg")
        .arg("-version")
        .output();

    match ffmpeg_check {
        Ok(_) => {},
        Err(_) => {
            println!("ffmpeg not found. Please install ffmpeg and ensure it is added to your PATH variable.");
            return;
        },
    }

    println!("ffmpeg found. Making frames...");
    let ffmpeg_make_frames = Command::new("ffmpeg")
        .arg("-i")
        .arg(&args[2])
        .arg("-vf")
        .arg("fps=3")
        .arg("./src/tmp/%08d.png")
        .output();

    match ffmpeg_make_frames {
        Ok(output) => {
            // print stderr
            if !output.stderr.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
            println!("ffmpeg finished making frames.");
        },
        Err(_) => {
            println!("ffmpeg failed to make frames.");
            return;
        },
    }

    // get frames
    let dir = fs::read_dir("./src/tmp/").
        expect("Error reading directory");
    let mut frames = Vec::new();
    for file in dir {
        if let Ok(entry) = file {
            frames.push(entry.path());
        }
    }
    
    // sort frames
    frames.sort();
    
    // print frames
    let term = termsize::get().unwrap();
    for frame in &frames {
        print_frame(&frame, &term);
    }

    // clear tmp dir
    // let _ = clear_dir();

}

fn print_frame(path: &std::path::Path, term: &termsize::Size) {
    let image = ImageReader::open(path);
    if image.is_err() {
        println!("Could not open image at {}", path.display());
        return;
    }
    let img = image.unwrap().decode().unwrap();

    let width_mul = (img.width() as f32 / term.cols as f32).ceil() as u32;
    let height_mul = (img.height() as f32 / term.rows as f32).ceil() as u32;

    let mut output = String::new();

    for y in (0..img.height()).step_by(height_mul as usize) {
        for x in (0..img.width()).step_by(width_mul as usize) {
            // Ensure we're within image bounds
            let safe_x = x.min(img.width() - 1);
            let safe_y = y.min(img.height() - 1);

            let pixel = img.get_pixel(safe_x, safe_y);

            let char = get_char((pixel[0] as i32 + pixel[1] as i32 + pixel[2] as i32) as f32 / (255.0*3.0));
            let color = RGB(pixel[0], pixel[1], pixel[2]);
            output.push_str(&format!("{0}", color.paint(char.to_string())));
        }
        output.push('\n');
    }

    // Use direct buffer writing for faster I/O
    use std::io::{self, Write};
    io::stdout().write_all(output.as_bytes()).unwrap();
    io::stdout().flush().unwrap();

    // Reset the cursor position to the top-left corner
    execute!(io::stdout(), MoveTo(0, 0)).unwrap();
}





fn get_char(luminance: f32) -> char {
    let luminance_arr = [0.0000, 0.0751, 0.0829, 0.0848, 0.1227, 0.1403, 0.1559, 0.185, 0.2183, 0.2417, 0.2571, 0.2852, 0.2902, 0.2919, 0.3099, 0.3192, 0.3232, 0.3294, 0.3384, 0.3609, 0.3619, 0.3667, 0.3737, 0.3747, 0.3838, 0.3921, 0.396, 0.3984, 0.3993, 0.4075, 0.4091, 0.4101, 0.42, 0.423, 0.4247, 0.4274, 0.4293, 0.4328, 0.4382, 0.4385, 0.442, 0.4473, 0.4477, 0.4503, 0.4562, 0.458, 0.461, 0.4638, 0.4667, 0.4686, 0.4693, 0.4703, 0.4833, 0.4881, 0.4944, 0.4953, 0.4992, 0.5509, 0.5567, 0.5569, 0.5591, 0.5602, 0.5602, 0.565, 0.5776, 0.5777, 0.5818, 0.587, 0.5972, 0.5999, 0.6043, 0.6049, 0.6093, 0.6099, 0.6465, 0.6561, 0.6595, 0.6631, 0.6714, 0.6759, 0.6809, 0.6816, 0.6925, 0.7039, 0.7086, 0.7235, 0.7302, 0.7332, 0.7602, 0.7834, 0.8037, 0.9999];
    
    // binary search nearest luminance
    let index = binary_search(luminance_arr, luminance);    

    let chars = " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";


    return chars.chars().nth(index).unwrap();
}

fn binary_search(arr: [f32; 92], target: f32) -> usize {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut closest_idx = left;

    while left <= right {
        let mid = (left + right) / 2;

        // Update closest_idx if this position is closer to target
        if (arr[mid] - target).abs() < (arr[closest_idx] - target).abs() {
            closest_idx = mid;
        }

        if arr[mid] < target {
            left = mid + 1;
        } else if arr[mid] > target {
            right = mid.checked_sub(1).unwrap_or(0); // Avoid underflow
        } else {
            break; // If it's an exact match, no need to continue
        }
    }

    closest_idx
}

fn clear_dir() -> std::io::Result<()> {
    let dir = fs::read_dir("./src/tmp/").
        expect("Error reading directory");
    for file in dir {
        fs::remove_file(file?.path())?;
    }
    Ok(())
}