// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

// use std::thread;

use std::thread;
use std::fs::File;
use std::io::{Write};
use std::path::Path;
use std::str::FromStr;
use serde::{Serialize, Deserialize};

pub fn init(){

    // thread::Builder::new().name("sprec_build".to_string()).spawn(move || {
    //     // loop{
    //     //     crate::sam::tools::linux_cmd(format!("python3 /opt/sam/scripts/sprec/build.py"));
    //     //     sleep(Duration::from_millis(100000));
    //     // }
    // });
    
}


pub fn build(){
    thread::spawn(move || {

        let mut pg_query = crate::sam::memory::PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(format!("HEARD")));
        pg_query.query_coulmns.push(format!("observation_type ="));

        
        let observations = crate::sam::memory::Observation::select(None, None, None, Some(pg_query)).unwrap();
        

        
        for observation in observations{
            log::info!("observation_humans: {:?}", observation.observation_humans);
            for human in observation.observation_humans{
                if !Path::new(format!("/opt/sam/scripts/sprec/audio/{}", human.oid).as_str()).exists(){
                    std::fs::create_dir(format!("/opt/sam/scripts/sprec/audio/{}", human.oid).as_str()).unwrap();
                }

                std::fs::write(format!("/opt/sam/scripts/sprec/audio/{}/{}.wav", human.oid, observation.oid).as_str(), observation.observation_file.clone().unwrap()).unwrap();
            }
        }



        crate::sam::tools::linux_cmd(format!("python3 /opt/sam/scripts/sprec/build.py"));

    });
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SprecPrediction {
    pub human: String,
    pub confidence: f64,
}

pub fn predict(file_path: &str) -> Result<SprecPrediction, crate::sam::services::Error>{
    if Path::new("/opt/sam/scripts/sprec/test.wav").exists(){
        std::fs::remove_file("/opt/sam/scripts/sprec/test.wav")?;
    }
    std::fs::copy(file_path, "/opt/sam/scripts/sprec/test.wav")?;
    let result = crate::sam::tools::cmd(format!("python3 /opt/sam/scripts/sprec/predict.py"));

    let mut split = result.split(":::::");
    let vec = split.collect::<Vec<&str>>();

    let sprec = SprecPrediction{
        human: vec[1].to_string(),
        confidence: f64::from_str(vec[2])?,
    };

    return Ok(sprec);
}


pub fn install() -> std::io::Result<()> {
    let data = include_bytes!("../../../scripts/sprec/build.py");

    let mut pos = 0;
    let mut buffer = File::create("/opt/sam/scripts/sprec/build.py")?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }

    let data = include_bytes!("../../../scripts/sprec/predict.py");

    let mut pos = 0;
    let mut buffer = File::create("/opt/sam/scripts/sprec/predict.py")?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }

    let data = include_bytes!("../../../scripts/sprec/requirements.txt");

    let mut pos = 0;
    let mut buffer = File::create("/opt/sam/scripts/sprec/requirements.txt")?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }

    let data = include_bytes!("../../../scripts/sprec/model.h5");

    let mut pos = 0;
    let mut buffer = File::create("/opt/sam/scripts/sprec/model.h5")?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }

    let data = include_bytes!("../../../scripts/sprec/labels.pickle");

    let mut pos = 0;
    let mut buffer = File::create("/opt/sam/scripts/sprec/labels.pickle")?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }

    let data = include_bytes!("../../../scripts/sprec/audio/Unknown.zip");

    let mut pos = 0;
    let mut buffer = File::create("/opt/sam/scripts/sprec/audio/Unknown.zip")?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }


    let data = include_bytes!("../../../scripts/sprec/noise/other.zip");

    let mut pos = 0;
    let mut buffer = File::create("/opt/sam/scripts/sprec/noise/other.zip")?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }

    let data = include_bytes!("../../../scripts/sprec/noise/_background_noise_.zip");

    let mut pos = 0;
    let mut buffer = File::create("/opt/sam/scripts/sprec/noise/_background_noise_.zip")?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }


    crate::sam::tools::extract_zip("/opt/sam/scripts/sprec/audio/Unknown.zip", format!("/opt/sam/scripts/sprec/audio/"));
    crate::sam::tools::extract_zip("/opt/sam/scripts/sprec/noise/other.zip", format!("/opt/sam/scripts/sprec/noise/"));
    crate::sam::tools::extract_zip("/opt/sam/scripts/sprec/noise/_background_noise_.zip", format!("/opt/sam/scripts/sprec/noise/"));

    crate::sam::tools::linux_cmd(format!("rm -rf /opt/sam/scripts/sprec/audio/Unknown.zip"));
    crate::sam::tools::linux_cmd(format!("rm -rf /opt/sam/scripts/sprec/noise/other.zip"));
    crate::sam::tools::linux_cmd(format!("rm -rf /opt/sam/scripts/sprec/noise/_background_noise_.zip"));


    log::info!("Installing requirements for sprec....");
    crate::sam::tools::linux_cmd(format!("pip3 install -r /opt/sam/scripts/sprec/requirements.txt"));
    
    log::info!("Building initial sprec model...");
    build();
    Ok(())
}