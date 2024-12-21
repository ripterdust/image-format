mod utils;
use utils::reader::Reader;

const IMAGE_PATH: &str = "path/to/image";
fn main() {
    let reader: Reader = Reader::new(IMAGE_PATH.to_string());

    reader.read();
}
