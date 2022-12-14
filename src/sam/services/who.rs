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

    let data = include_bytes!("../../../scripts/who.io/who2.py");
    let mut pos = 0;
    let mut buffer = File::create("/opt/sam/scripts/who.io/who2.py")?;
    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }


    let data = include_bytes!("../../../scripts/who.io/trained_knn_model.clf");
    let mut pos = 0;
    let mut buffer = File::create("/opt/sam/scripts/who.io/trained_knn_model.clf")?;
    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }

    let data = include_bytes!("../../../scripts/who.io/dataset/barack_obama.zip");
    let mut pos = 0;
    let mut buffer = File::create("/opt/sam/scripts/who.io/dataset/barack_obama.zip")?;
    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }

    let data = include_bytes!("../../../scripts/who.io/dataset/donald_trump.zip");
    let mut pos = 0;
    let mut buffer = File::create("/opt/sam/scripts/who.io/dataset/donald_trump.zip")?;
    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }

    crate::sam::tools::extract_zip("/opt/sam/scripts/who.io/dataset/barack_obama.zip", format!("/opt/sam/scripts/who.io/dataset/"));
    crate::sam::tools::extract_zip("/opt/sam/scripts/who.io/dataset/donald_trump.zip", format!("/opt/sam/scripts/who.io/dataset/"));
    crate::sam::tools::linux_cmd(format!("rm -rf /opt/sam/scripts/who.io/dataset/barack_obama.zip"));
    crate::sam::tools::linux_cmd(format!("rm -rf /opt/sam/scripts/who.io/dataset/donald_trump.zip"));

    Ok(())

}