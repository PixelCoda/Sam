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
use rouille::post_input;

pub fn handle(current_session: crate::sam::memory::WebSessions, request: &Request) -> Result<Response, crate::sam::http::Error> {
    
    
    
    if request.url() == "/api/services/notifications/unseen" {
        if request.method() == "GET" {
            let mut pg_query = crate::sam::memory::PostgresQueries::default();
            pg_query.queries.push(crate::sam::memory::PGCol::Boolean(false));
            pg_query.query_coulmns.push(format!("seen ="));

            pg_query.queries.push(crate::sam::memory::PGCol::String(current_session.human_oid));
            pg_query.query_coulmns.push(format!(" AND human_oid ="));

            let notifications = crate::sam::memory::Notification::select(None, None, Some(format!("timestamp DESC")), Some(pg_query))?;
            
            return Ok(Response::json(&notifications));
        }
    }

    if request.url() == "/api/services/notifications/seen" {
        let input = post_input!(request, {
            oid: String
        })?;
        let mut pg_query = crate::sam::memory::PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(input.oid.clone()));
        pg_query.query_coulmns.push(format!("oid ="));

        let notifications = crate::sam::memory::Notification::select(Some(20), None, Some(format!("timestamp DESC")), Some(pg_query))?;
        let mut notification = notifications[0].clone();
        notification.seen = true;
        notification.save().unwrap();

        return Ok(Response::json(&notification));
    }
    
    
    if request.url() == "/api/services/notifications" {

        if request.method() == "GET" {
            let mut pg_query = crate::sam::memory::PostgresQueries::default();

            pg_query.queries.push(crate::sam::memory::PGCol::String(current_session.human_oid));
            pg_query.query_coulmns.push(format!("human_oid ="));

            let notifications = crate::sam::memory::Notification::select(Some(20), None, Some(format!("timestamp DESC")), Some(pg_query))?;
            
            return Ok(Response::json(&notifications));
        }

        if request.method() == "POST" {
            let input = post_input!(request, {
                message: String
            })?;


            let mut notification = crate::sam::memory::Notification::new();
            notification.message = input.message;
            notification.sid = current_session.sid;
            notification.human_oid = current_session.human_oid;
            notification.save().unwrap();

            return Ok(Response::json(&notification));

        }


    }
    return Ok(Response::empty_404());
}