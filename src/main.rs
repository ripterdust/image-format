mod utils;
use utils::reader::Reader;

const IMAGE_PATH: &str = "./download.jpg";
fn main() {
    let mut reader: Reader = Reader::new(IMAGE_PATH.to_string());

    reader.convert();

    println!("Image read successfully!");
}
