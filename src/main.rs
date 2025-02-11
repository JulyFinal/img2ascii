use image::{GenericImageView, DynamicImage};
use colored::*;
use std::path::Path;
use clap::{Parser, ValueEnum};
use rayon::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum CliSize {
    Small,
    Medium,
    Large,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum CliMode {
    Monochrome,
    Color,
}

#[derive(Clone)]
enum Size {
    Small,
    Medium,
    Large,
}

#[derive(Clone)]
enum OutputMode {
    Monochrome,
    Color,
}

struct AsciiConverter {
    image: DynamicImage,
    size: Size,
    mode: OutputMode,
    custom_width: Option<u32>,
    char_ratio: f32,
}

impl AsciiConverter {
    fn new_with_width(
        image_path: &Path,
        size: Size,
        mode: OutputMode,
        custom_width: Option<u32>,
        char_ratio: Option<f32>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let image = image::open(image_path)?;
        Ok(Self {
            image,
            size,
            mode,
            custom_width,
            char_ratio: char_ratio.unwrap_or(0.5),
        })
    }

    fn get_dimensions(&self) -> (u32, u32) {
        let (original_width, original_height) = self.image.dimensions();

        let base_width = if let Some(width) = self.custom_width {
            width
        } else {
            match self.size {
                Size::Small => 60,
                Size::Medium => 100,
                Size::Large => 150,
            }
        };

        let target_width = base_width;
        let target_height = ((original_height as f32 / original_width as f32)
            * (base_width as f32)
            * self.char_ratio) as u32;

        (target_width, target_height)
    }

    fn pixel_to_ascii(&self, brightness: u8) -> char {
        const ASCII_CHARS: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
        let index = (brightness as usize * (ASCII_CHARS.len() - 1)) / 255;
        ASCII_CHARS[index]
    }

    fn convert(&self) -> String {
        let (target_width, target_height) = self.get_dimensions();
        let resized = self.image.resize_exact(
            target_width,
            target_height,
            image::imageops::FilterType::Nearest
        );

        let capacity = (target_width as usize + 1) * target_height as usize;
        let mut result = String::with_capacity(capacity);

        let pixels: Vec<_> = (0..target_height)
            .flat_map(|y| (0..target_width).map(move |x| (x, y)))
            .collect();

        let ascii_lines: Vec<String> = pixels
            .par_chunks(target_width as usize)
            .map(|chunk| {
                let mut line = String::with_capacity(target_width as usize + 1);
                for &(x, y) in chunk {
                    let pixel = resized.get_pixel(x, y);
                    let brightness = ((pixel[0] as f32 * 0.299) +
                                    (pixel[1] as f32 * 0.587) +
                                    (pixel[2] as f32 * 0.114)) as u8;

                    let ascii_char = self.pixel_to_ascii(brightness);

                    match self.mode {
                        OutputMode::Monochrome => {
                            line.push(ascii_char);
                        },
                        OutputMode::Color => {
                            let colored_char = format!("{}", ascii_char)
                                .truecolor(pixel[0], pixel[1], pixel[2]);
                            line.push_str(&colored_char.to_string());
                        }
                    }
                }
                line
            })
            .collect();

        for line in ascii_lines {
            result.push_str(&line);
            result.push('\n');
        }

        result
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the image file
    #[arg(short, long)]
    image: String,

    /// Output size (small, medium, large)
    #[arg(short, long, value_enum, default_value_t = CliSize::Medium)]
    size: CliSize,

    /// Output mode (monochrome, color)
    #[arg(short, long, value_enum, default_value_t = CliMode::Color)]
    mode: CliMode,

    /// Custom width (optional, overrides size parameter)
    #[arg(short = 'w', long)]
    width: Option<u32>,

    /// Character aspect ratio adjustment (default: 0.5)
    #[arg(short = 'r', long, default_value_t = 0.5)]
    char_ratio: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let size = match cli.size {
        CliSize::Small => Size::Small,
        CliSize::Medium => Size::Medium,
        CliSize::Large => Size::Large,
    };

    let mode = match cli.mode {
        CliMode::Monochrome => OutputMode::Monochrome,
        CliMode::Color => OutputMode::Color,
    };

    let image_path = Path::new(&cli.image);
    let converter = AsciiConverter::new_with_width(
        image_path,
        size,
        mode,
        cli.width,
        Some(cli.char_ratio)
    )?;

    let ascii_art = converter.convert();
    println!("{}", ascii_art);

    Ok(())
}
