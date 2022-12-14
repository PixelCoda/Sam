// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

use rouille::post_input;
use rouille::Request;
use rouille::Response;


pub fn handle(_current_session: crate::sam::memory::WebSessions, request: &Request) -> Result<Response, crate::sam::http::Error> {
    if request.url() == "/api/locations" {
        let objects = crate::sam::memory::Location::select(None, None, None, None)?;
        return Ok(Response::json(&objects));
    }

    if request.url().contains("/api/locations") && request.url().contains("/rooms") {
       
        let url = request.url().clone();
        let split = url.split("/");
        let vec = split.collect::<Vec<&str>>();
        let location_oid = vec[3];

        if request.method() == "GET" {
           
        

            let mut pg_query = crate::sam::memory::PostgresQueries::default();
            pg_query.queries.push(crate::sam::memory::PGCol::String(location_oid.clone().to_string()));
            pg_query.query_coulmns.push(format!("location_oid ="));

            let rooms = crate::sam::memory::Room::select(None, None, None, Some(pg_query))?;
        
            
            return Ok(Response::json(&rooms));
        }

        if request.method() == "POST" {
            let input = post_input!(request, {
                name: String
            })?;

            let mut room = crate::sam::memory::Room::new();
            room.name = input.name;
            room.location_oid = location_oid.to_string();
            room.save().unwrap();

            let mut pg_query = crate::sam::memory::PostgresQueries::default();
            pg_query.queries.push(crate::sam::memory::PGCol::String(room.oid.clone()));
            pg_query.query_coulmns.push(format!("oid ="));

            let objects = crate::sam::memory::Room::select(None, None, None, Some(pg_query))?;
            if objects.len() > 0 {
                if request.url().contains(".json"){
                    return Ok(Response::json(&objects[0]));
                } else {
                    let response = Response::redirect_302("/locations.html");
                    return Ok(response);
                }
                
            } else {
                return Ok(Response::empty_404());
            }
           

        }
    }

    return Ok(Response::empty_404());
}