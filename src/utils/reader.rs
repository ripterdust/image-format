pub struct Reader {
    image_path: String,
}

impl Reader {
    pub fn new(path: String) -> Reader {
        return Reader { image_path: path };
    }

    pub fn read(&self) {
        println!("Reading...");
    }
}
