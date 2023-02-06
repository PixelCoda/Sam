pub mod games;
pub mod image;
pub mod snapcast;
pub mod youtube;


use std::fs;
use std::fs::File;
use std::io::{Write};

use rouille::post_input;
use rouille::Request;
use rouille::Response;



pub fn install() -> std::io::Result<()> {

    match games::install(){
        Ok(_) => {
            log::info!("Games installed successfully");
        },
        Err(e) => {
            log::error!("Failed to install games: {}", e);
        }
    }

    match snapcast::install(){
        Ok(_) => {
            log::info!("Snapcast server installed successfully");
        },
        Err(e) => {
            log::error!("Failed to install snapcast server: {}", e);
        }
    }


    match image::install(){
        Ok(_) => {
            log::info!("Image service installed successfully");
        },
        Err(e) => {
            log::error!("Failed to install image service: {}", e);
        }
    }


    return Ok(());
}

pub fn handle(current_session: crate::sam::memory::WebSessions, request: &Request) -> Result<Response, crate::sam::http::Error> {
    if request.url().contains("/image"){
        return image::handle(current_session, request);
    }

    if request.url().contains("/games"){
        return games::handle(current_session, request);
    }

    return Ok(Response::empty_404());
}
