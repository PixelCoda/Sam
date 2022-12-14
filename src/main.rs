// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

// sam
pub mod sam;

// import external crates
extern crate wikipedia;
extern crate hound;
extern crate postgres;

// store application version as a const
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

// main
#[tokio::main]
async fn main() {
    log::info!("███████     █████     ███    ███    ");
    log::info!("██         ██   ██    ████  ████    ");
    log::info!("███████    ███████    ██ ████ ██    ");
    log::info!("     ██    ██   ██    ██  ██  ██    ");
    log::info!("███████ ██ ██   ██ ██ ██      ██ ██ ");
    log::info!("Smart Artificial Mind");
    log::info!("VERSION: {:?}", VERSION);

    sudo::with_env(&["LIBTORCH", "LD_LIBRARY_PATH", "PG_DBNAME", "PG_USER", "PG_PASS", "PG_ADDRESS"]).unwrap();
    // sudo::escalate_if_needed().unwrap();

    simple_logger::SimpleLogger::new().with_colors(true).init().unwrap();

    crate::sam::setup::install().await;

    let config = crate::sam::memory::Config::new();

    config.init().await;

    // Initialize Snapcast Server
    crate::sam::services::snapcast::init();

    // Initialize Web Socket Server
    crate::sam::services::socket::init();

    // Initialize RTSP Service
    crate::sam::services::rtsp::init();

    // Initialize Sound Service
    crate::sam::services::sound::init();
    
    // Syncs database with Lifx API
    crate::sam::services::lifx::ssync();

    // Initialize default settings
    crate::sam::http::api::settings::set_defaults();
    
    // Configure Snapcast
    crate::sam::services::snapcast::configure();

    // Initialize Storage Service
    crate::sam::services::storage::init();




    

    loop {}
}
