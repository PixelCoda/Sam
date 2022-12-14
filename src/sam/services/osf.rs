/// ███████     █████     ███    ███    
/// ██         ██   ██    ████  ████    
/// ███████    ███████    ██ ████ ██    
///      ██    ██   ██    ██  ██  ██    
/// ███████ ██ ██   ██ ██ ██      ██ ██ 
/// Copyright 2021-2022 The Open Sam Foundation (OSF)
/// Developed by Caleb Mitchell Smith (PixelCoda)
/// Licensed under GPLv3....see LICENSE file.

/// Jupiter.rs acts as a service module to connect sam to your jupiter server 
/// Jupiter is a rust based wether server devloped by The Open Sam Foundation 
/// and can be found in the foundations git repository.

use rouille::Request;
use rouille::Response;
use serde::{Serialize, Deserialize};



pub fn handle(_current_session: crate::sam::memory::WebSessions, request: &Request) -> Result<Response, crate::sam::http::Error> {
    if request.url() == "/api/services/osf/packages" {
        let jupiter = crate::sam::services::osf::get();
        match jupiter{
            Ok(j) => {
                return Ok(Response::json(&j));
            },
            Err(e) => {
                return Ok(Response::text(&e.to_string()));
            }
        }
    }
    return Ok(Response::empty_404());
}



/// curl -X GET "https://jupiter.alpha.opensam.foundation/" -H "Authorization: xxx"
pub fn get() -> Result<Packages, crate::sam::services::Error> { 
    let request = reqwest::blocking::Client::new().get("https://osf.opensam.foundation/api/packages").send()?;
    let json = request.json::<Packages>()?;
    return Ok(json);
}

pub type Packages = Vec<Package>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    pub name: String, // unique
    pub versions: Vec<String>,
    pub category_tags: Vec<String>,
    pub icon_base64: String,
    pub latest_version: String,
    pub latest_oid: String,
}