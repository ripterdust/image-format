mod utils;
use utils::{converter::Converter, reader::Reader};

const IMAGE_PATH: &str = "./download.jpg";
fn main() {
    let mut converter: Converter = Converter::new(IMAGE_PATH.to_string());

    converter.convert();

    let reader: Reader = Reader::new();

    reader.read();
}
