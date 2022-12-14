// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

use rouille::Request;
use rouille::Response;
use std::thread;
use std::time::Duration;


pub fn handle(_current_session: crate::sam::memory::WebSessions, request: &Request) -> Result<Response, crate::sam::http::Error> {
    if request.url() == "/api/services/tts" {
        let input = request.get_param("text").unwrap();
        return Ok(Response::from_data("audio/wav", crate::sam::services::tts::get(input).unwrap()));
        
    }
    return Ok(Response::empty_404());
}

pub fn init(){

    let tts_thead = thread::Builder::new().name("mozillatts".to_string()).spawn(move || {
        crate::sam::tools::linux_cmd(format!("docker run -p 5002:5002 synesthesiam/mozillatts"));
    });
    match tts_thead{
        Ok(_) => {
            log::info!("tts server started successfully");
        },
        Err(e) => {
            log::error!("failed to initialize tts server: {}", e);
        }
    }
}

pub fn get(text: String) -> Result<Vec<u8>, crate::sam::services::Error> {

    match fetch_online(text.clone()) {
        Ok(x) => {
            return Ok(x);
        },
        Err(_e) => {

            match fetch_local(text.clone()) {
                Ok(x) => {
                    return Ok(x);
                },
                Err(e) => {
                    return Err(e);
                }
            }

            
        }
    }

}

pub fn fetch_online(text: String) -> Result<Vec<u8>, crate::sam::services::Error> {
    let client = reqwest::blocking::Client::new();
    let bytes = client.get(format!("https://tts.opensam.foundation/api/tts?text={}&speaker_id=&style_wav=", text))
        .basic_auth("sam", Some("87654321"))
        .timeout(Duration::from_secs(5))
        .send()?.bytes()?;
    Ok(bytes.to_vec())
}

pub fn fetch_local(text: String) -> Result<Vec<u8>, crate::sam::services::Error> {
    let client = reqwest::blocking::Client::new();
    let bytes = client.get(format!("http://localhosy:5002/api/tts?text={}&speaker_id=&style_wav=", text))
        .timeout(Duration::from_secs(5))
        .send()?.bytes()?;
    Ok(bytes.to_vec())
}