// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

use error_chain::error_chain;
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        Postgres(postgres::Error);
        Hound(hound::Error);
        PostError(rouille::input::post::PostError);
        ParseFloatError(std::num::ParseFloatError);
        TchError(tch::TchError);
    }
}

pub mod darknet;
pub mod dropbox;
pub mod image;
pub mod youtube;
pub mod jupiter;
pub mod lifx;
pub mod notifications;
pub mod osf;
pub mod rivescript;
pub mod rtsp;
pub mod snapcast;
pub mod socket;
pub mod sound;
pub mod sprec;
pub mod storage;
pub mod stt;
pub mod tts;
pub mod who;