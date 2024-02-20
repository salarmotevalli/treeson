use std::fs::{metadata, File};
pub(crate) use std::io::{self, Read};
use std::os::unix::{io::AsRawFd, prelude::MetadataExt};

pub(crate) fn is_data_piped() -> bool {
    let fd = io::stdin().as_raw_fd();
    let meta = metadata("/dev/fd/".to_owned() + &fd.to_string());

    match meta {
        Ok(meta) => meta.mode() == 4480, // Return is data piped
        Err(_) => false,
    }
}

pub(crate) fn define_reader(std_in: Option<String>) -> Box<dyn Read> {
    if let Some(input) = std_in {
        let file = File::open(input).expect("cannot open file");
        Box::new(file)
    } else {
        Box::new(io::stdin())
    }
}
