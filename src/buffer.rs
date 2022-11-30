use std::error;
use std::fs;
use std::io;
use std::io::Read;
use std::path;
use std::vec;

pub type Buffer = io::Cursor<vec::Vec<u8>>;

pub struct Data {
    pub buffer: io::Cursor<vec::Vec<u8>>,
}

impl Data {
    pub fn new() -> Self {
        return Data {
            buffer: io::Cursor::new(vec::Vec::new()),
        };
    }
    pub fn from_path<P: AsRef<path::Path>>(p: P) -> Result<Self, Box<dyn error::Error>> {
        let file = fs::File::open(p)?;
        return Self::from_file(file);
    }

    pub fn from_file(mut file: fs::File) -> Result<Self, Box<dyn error::Error>> {
        let mut data = Self::new();

        file.read_to_end(data.buffer.get_mut())?;

        return Ok(data);
    }
}
