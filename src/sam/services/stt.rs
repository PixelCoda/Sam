// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.


// TODO - Ability to use multiple stt servers
// Cloud -> Internal Cloud -> Localhost
// TODO - Don't start docker unless localhost has been called

use rouille::Request;
use rouille::Response;
use rouille::post_input;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct STTPrediction {
    pub stt: String,
    pub human: String,
    pub confidence: f64,
}

// TODO - Fallback to STT api if whisper fails
// TODO - Return defult unknown if sprec fails
pub fn process(file_path: String) -> Result<STTPrediction, crate::sam::services::Error> {
    let whisper = crate::sam::services::stt::whisper(file_path.clone())?;
    let sprec = crate::sam::services::sprec::predict(&file_path)?;

    return Ok(STTPrediction{
        stt: whisper,
        human: sprec.human,
        confidence: sprec.confidence,
    });
}

// /opt/sam/bin/whisper -m /opt/sam/models/ggml-base.en.bin -f ./output.wav -otxt
pub fn whisper(file_path: String) -> Result<String, crate::sam::services::Error> {
   
    crate::sam::tools::linux_cmd(format!("ffmpeg -i {} -ar 16000 -ac 1 -c:a pcm_s16le {}.16.wav", file_path, file_path));

    crate::sam::tools::linux_cmd(format!("/opt/sam/bin/whisper -m /opt/sam/models/ggml-base.en.bin -f {}.16.wav -otxt", file_path));
    
    let data = std::fs::read_to_string(format!("{}.16.wav.txt", file_path).as_str())?;


    std::fs::remove_file(format!("{}.16.wav", file_path).as_str())?;
    std::fs::remove_file(format!("{}.16.wav.txt", file_path).as_str())?;

    return Ok(data);
}

pub fn patch_whisper_wts(file_path: String) -> Result<(), crate::sam::services::Error>{
    let mut data = std::fs::read_to_string(format!("{}", file_path).as_str())?;
    data = data.replace("ffmpeg", "/opt/sam/bin/ffmpeg").replace("/System/Library/Fonts/Supplemental/Courier New Bold.ttf","/opt/sam/fonts/courier.ttf");
    std::fs::remove_file(format!("{}", file_path).as_str())?;
    std::fs::write(file_path, data)?;
    return Ok(());
}


// TODO: Compile whisper for raspi and patch installer
pub fn install() -> std::io::Result<()> {
    let data = include_bytes!("../../../packages/whisper/models/ggml-base.en.bin");

    let mut pos = 0;
    let mut buffer = File::create("/opt/sam/models/ggml-base.en.bin")?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }

    let data = include_bytes!("../../../packages/whisper/main-amd64");

    let mut pos = 0;
    let mut buffer = File::create("/opt/sam/bin/whisper")?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }

    crate::sam::tools::linux_cmd(format!("chmod +x /opt/sam/bin/whisper"));
    Ok(())
}








pub fn handle(_current_session: crate::sam::memory::WebSessions, request: &Request) -> Result<Response, crate::sam::http::Error> {
    
   

    
    if request.url() == "/api/services/stt" {


        let data = post_input!(request, {
            audio_data: rouille::input::post::BufferedFile,
        })?;


        let tmp_file_path = format!("/opt/sam/tmp/{}.wav", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64);

        let mut file = File::create(tmp_file_path.clone()).unwrap();
        file.write_all(&data.audio_data.data).unwrap();


        let mut idk = crate::sam::services::stt::upload(tmp_file_path).unwrap();

        // TODO - Spawn thread to store audio/text files as an observation.
        // TODO - Spawn sprec thread to identify speaker.
        // TODO - Spawn thread to process sam brain.py. (maybe, might execute in js runtime instead)
        
        
        // If idk.text contains "sam" then redirect request to io api
        if idk.text.contains("sam") {
            return Ok(Response::redirect_303(format!("/api/io?input={}", idk.text.replace("sam ", ""))));
        }

        idk.response_type = Some(format!("stt"));

        return Ok(Response::json(&idk));
    }



    
    return Ok(Response::empty_404());
}

pub fn init(){

    let stt_thread = thread::Builder::new().name("stt".to_string()).spawn(move || {
        crate::sam::tools::linux_cmd(format!("docker run -p 8002:8000 p0indexter/stt"));
    });
    match stt_thread{
        Ok(_) => {
            log::info!("stt server started successfully");
        },
        Err(e) => {
            log::error!("failed to initialize stt server: {}", e);
        }
    }
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct STTReply {
    pub text: String,
    pub time: f64,
    pub response_type: Option<String>,
}


// TODO - Use foundation stt public server first, fallback to local server after 2 seconds for offline avaliablity
// TODO - Find another method besided multipart....too many dependencies
pub fn upload(_tmp_file_path: String) -> Result<STTReply, crate::sam::services::Error> {

    return Ok(STTReply{
        text: String::new(),
        time: 0.0,
        response_type: None,
    });

    // let form = reqwest::blocking::multipart::Form::new().file("speech", tmp_file_path.as_str())?;


    // let client = reqwest::blocking::Client::new();

    // Ok(client.post(format!("https://stt.opensam.foundation/api/v1/stt"))
    // .multipart(form)
    // .send()?.json()?)

    // Ok(client.post(format!("http://localhost:8002/api/v1/stt"))
    //     .multipart(form)
    //     .send()?.json()?)
}