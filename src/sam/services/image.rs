pub mod nst;
pub mod srgan;


use std::fs;
use std::fs::File;
use std::io::{Write};


pub fn install() -> std::io::Result<()> {
    let data = include_bytes!("../../../packages/tch/vgg16.ot");

    let mut pos = 0;
    let mut buffer = File::create("/opt/sam/models/vgg16.ot")?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }

    Ok(())
}
