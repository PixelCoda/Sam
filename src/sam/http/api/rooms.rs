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

pub fn handle(_current_session: crate::sam::memory::WebSessions, request: &Request) -> Result<Response, crate::sam::http::Error> {
    
    if request.url().contains("/api/rooms") && request.url().contains("/things") {
       
        let url = request.url().clone();
        let split = url.split("/");
        let vec = split.collect::<Vec<&str>>();
        let room_oid = vec[3];

        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct WebThing {
            pub id: i32,
            pub oid: String,
            pub name: String,
            pub room: Option<crate::sam::memory::Room>,
            pub thing_type: String, // lifx, etc
            pub online_identifiers: Vec<String>,
            pub local_identifiers: Vec<String>,
            pub created_at: i64,
            pub updated_at: i64
        }

        let mut webthings: Vec<WebThing> = Vec::new();

        let mut pg_query = crate::sam::memory::PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(room_oid.clone().to_string()));
        pg_query.query_coulmns.push(format!("room_oid ="));
        let objects = crate::sam::memory::Thing::select(None, None, None, Some(pg_query))?;
        
        for object in objects{

            let mut room: Option<crate::sam::memory::Room> = None;
            let mut pg_query = crate::sam::memory::PostgresQueries::default();
            pg_query.queries.push(crate::sam::memory::PGCol::String(object.room_oid.clone()));
            pg_query.query_coulmns.push(format!("oid ="));
            let rooms = crate::sam::memory::Room::select(None, None, None, Some(pg_query));
            match rooms{
                Ok(r) => {
                    if r.len() > 0 {
                        room = Some(r[0].clone());
                    }
                },
                Err(e) => {
                    log::error!("{}", e);
                }
            }


            let web_thing = WebThing{
                id: object.id,
                oid: object.oid,
                name: object.name,
                room: room,
                thing_type: object.thing_type,
                online_identifiers: object.online_identifiers,
                local_identifiers: object.local_identifiers,
                created_at: object.created_at,
                updated_at: object.updated_at
            };
            webthings.push(web_thing);
        }
        
        return Ok(Response::json(&webthings));
    }


    if request.url() == "/api/rooms" {
        let objects = crate::sam::memory::Room::select(None, None, None, None)?;
        return Ok(Response::json(&objects));
    }

    return Ok(Response::empty_404());
}