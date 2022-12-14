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
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IOReply {
    pub text: String,
    pub timestamp: i64,
    pub response_type: String,
}


pub fn handle(_current_session: crate::sam::memory::WebSessions, request: &Request) -> Result<Response, crate::sam::http::Error> {
   
    let input = request.get_param("input");

    match input{
        Some(iput) => {
            
            let rivescript_reply = crate::sam::tools::cmd(format!("python3 /opt/sam/scripts/rivescript/brain.py \"{}\"", iput));
        
            if rivescript_reply.contains(":::::"){
                // TODO - Parse Command
            } 
        
            let io = IOReply{
                text: rivescript_reply,
                timestamp: 0,
                response_type: format!("io")
            };
        
            return Ok(Response::json(&io));
        },
        None => {
            let response = Response::text("IO input malformed").with_status_code(500);
            return Ok(response);
        }
    }
   
}