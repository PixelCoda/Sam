// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

use std::fs;
use std::fs::File;
use std::io::{Write};


use std::process::{Command, Stdio};


// ./darknet detect cfg/yolov3-tiny.cfg yolov3-tiny.weights data/dog.jpg
// {id: "dog", probability: 0.570732, left: 129, right: 369, top: 186, bottom: 517}
// {id: "car", probability: 0.517267, left: 533, right: 621, top: 94, bottom: 157}
// {id: "car", probability: 0.615291, left: 465, right: 679, top: 71, bottom: 169}
// {id: "bicycle", probability: 0.585022, left: 206, right: 575, top: 150, bottom: 450}
pub fn darknet_image_with_gpu(file_path: String) -> Result<String, String> {


    let _observation_file = fs::read(file_path.as_str()).unwrap();


    let child = Command::new("sh")
    .arg("-c")
    .arg(format!("cd /opt/sam/bin/darknet/ && ./darknet-gpu detect /opt/sam/bin/darknet/cfg/yolov3-tiny.cfg /opt/sam/bin/darknet/yolov3-tiny.weights {}", file_path.clone()))
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");

    let output = child
        .wait_with_output()
        .expect("failed to wait on child");
    let darknet = String::from_utf8_lossy(&output.stdout).to_string().replace("\n", "");

    if darknet.to_lowercase().contains("error") || darknet.len() == 0 {
        return Err(format!("deepvision_scan_image_with_cpu_error: {}", darknet))
    }

    return Ok(darknet);

}

pub fn install() -> std::io::Result<()> {
    let data = include_bytes!("../../../scripts/darknet/darknet.zip");

    let mut pos = 0;
    let mut buffer = File::create("/opt/sam/bin/darknet.zip")?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }

    crate::sam::tools::extract_zip("/opt/sam/bin/darknet.zip", format!("/opt/sam/bin/"));
    crate::sam::tools::linux_cmd(format!("rm -rf /opt/sam/bin/darknet.zip"));
    crate::sam::tools::linux_cmd(format!("chmod +x /opt/sam/bin/darknet/darknet"));
    Ok(())
}
