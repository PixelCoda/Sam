pub mod humans;
pub mod io;
pub mod locations;
pub mod observations;
pub mod pets;
pub mod services;
pub mod things;
pub mod rooms;
pub mod settings;

use rouille::Request;
use rouille::Response;

pub fn handle_api_request(current_session: crate::sam::memory::WebSessions, request: &Request) -> Result<Response, crate::sam::http::Error> {
   
    if request.url() == "/api/sid" {
        return Ok(Response::text(current_session.sid));
    }
    
    // TODO: Fetch Human and append to response
    if request.url() == "/api/current_session" {
        return Ok(Response::json(&current_session));
    }

    if request.url() == "/api/current_human" {

        // Search for OID matches
        let mut pg_query = crate::sam::memory::PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(current_session.human_oid.clone()));
        pg_query.query_coulmns.push(format!("oid ="));

        let human = crate::sam::memory::Human::select(None, None, None, Some(pg_query))?;
        return Ok(Response::json(&human[0]));
    }

    // IO: GET
    if request.url().contains("/api/io"){
        return Ok(io::handle(current_session, request)?);
    }

    if request.url().contains("/api/humans"){
        return Ok(humans::handle(current_session, request)?);
    }

    if request.url().contains("/api/locations"){
        return Ok(locations::handle(current_session, request)?);
    }

    if request.url().contains("/api/observations"){
        return Ok(observations::handle(current_session, request)?);
    }

    if request.url().contains("/api/rooms"){
        return Ok(rooms::handle(current_session, request)?);
    }

    if request.url().contains("/api/services"){
        return Ok(services::handle(current_session, request)?);
    }

    if request.url().contains("/api/settings"){
        return Ok(settings::handle(current_session, request)?);
    }

    if request.url().contains("/api/things"){
        return Ok(things::handle(current_session, request)?);
    }
    

    return Ok(Response::empty_404());
}