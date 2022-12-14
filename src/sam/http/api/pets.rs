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

pub fn handle(_current_session: crate::sam::memory::WebSessions, _request: &Request) -> Result<Response, crate::sam::http::Error> {
    return Ok(Response::empty_404());
}