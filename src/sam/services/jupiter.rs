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

pub fn get_db_obj() -> Result<crate::sam::memory::Service, crate::sam::services::Error>{
    let mut pg_query = crate::sam::memory::PostgresQueries::default();
    pg_query.queries.push(crate::sam::memory::PGCol::String(format!("jupiter")));
    pg_query.query_coulmns.push(format!("identifier ="));
    let service = crate::sam::memory::Service::select(None, None, None, Some(pg_query))?;
    return Ok(service[0].clone());
}

pub fn handle(_current_session: crate::sam::memory::WebSessions, request: &Request) -> Result<Response, crate::sam::http::Error> {
    if request.url() == "/api/services/jupiter" {
        let jupiter = crate::sam::services::jupiter::get();
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
pub fn get() -> Result<CachedWeatherData, crate::sam::services::Error> {
    let jupiter_config = get_db_obj()?;    
    let request = reqwest::blocking::Client::new().get(jupiter_config.endpoint).header("Authorization", format!("Bearer {}", jupiter_config.secret)).send()?;
    let json = request.json::<CachedWeatherData>()?;
    return Ok(json);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CachedWeatherData {
    pub id: i32,
    pub oid: String,
    pub accuweather: Option<String>, // JSON string
    pub homebrew: Option<String>, // JSON string
    pub openweathermap: Option<String>, // JSON string
    pub timestamp: i64
}