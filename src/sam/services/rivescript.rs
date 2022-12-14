// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

use std::fs::File;
use std::io::{Write};

pub fn install() -> std::io::Result<()> {
    let data = include_bytes!("../../../scripts/rivescript/brain.py");

    let mut pos = 0;
    let mut buffer = File::create("/opt/sam/scripts/rivescript/brain.py")?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }

    let data = include_bytes!("../../../scripts/rivescript/eg.zip");

    let mut pos = 0;
    let mut buffer = File::create("/opt/sam/scripts/rivescript/eg.zip")?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }

    crate::sam::tools::extract_zip("/opt/sam/scripts/rivescript/eg.zip", format!("/opt/sam/scripts/rivescript/"));
    crate::sam::tools::linux_cmd(format!("rm -rf /opt/sam/scripts/rivescript/eg.zip"));

    Ok(())
}
