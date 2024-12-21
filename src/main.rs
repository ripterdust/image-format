mod utils;
use utils::converter::Converter;

const IMAGE_PATH: &str = "./download.jpg";
fn main() {
    let mut reader: Converter = Converter::new(IMAGE_PATH.to_string());

    reader.convert();

    println!("Image read successfully!");
}
