mod ascii_image;

const DEFAULT_SIZE: u32 = 100;
const INVALID_ARG_MESSAGE: &str =
    "Invalid arguments! Try `binary-artist --img <path> <width> <height>` \
                                  or `binary-artist --txt <text> <width> <height>`";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut is_success = false;

    if args.len() > 4 {
        match args[1].as_str() {
            "--img" => {
                let mut converter = ascii_image::Converter::new();
                let ascii_img = converter
                    .set_img(args[2].as_str())
                    .resize(ascii_image::ImgSize {
                        width: args[3].parse::<u32>().unwrap_or(DEFAULT_SIZE),
                        height: args[4].parse::<u32>().unwrap_or(DEFAULT_SIZE),
                    })
                    .execute();
                if ascii_img.is_ok() {
                    println!("{}", ascii_img.unwrap());
                    is_success = true;
                }
            }
            _ => {}
        }
    }
    if !is_success {
        println!("{}", INVALID_ARG_MESSAGE);
    }
}
