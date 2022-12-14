// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

extern crate lifx_api_server;
extern crate lifx_rs as lifx;

use online::check;
use rouille::Request;
use rouille::Response;
use rouille::post_input;
use std::thread;

pub fn init_server(key: String) {
    let lifx_thread = thread::Builder::new().name("lifx_api_server".to_string()).spawn(move || {
        let config = lifx_api_server::Config { 
            secret_key: key,
            port: 7084
        };

        lifx_api_server::start(config);

        loop {
            
        }

    });

    match lifx_thread{
        Ok(_) => {
            log::info!("lifx api server started successfully");
        },
        Err(e) => {
            log::error!("failed to initialize lifx api server: {}", e);
        }
    }
}

pub fn ssync(){
    thread::spawn(move || {
        let mut pg_query = crate::sam::memory::PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(format!("lifx")));
        pg_query.query_coulmns.push(format!("identifier ="));
        let services = crate::sam::memory::Service::select(None, None, None, Some(pg_query));

        match services {
            Ok(services) => {
                // crate::sam::services::lifx::init_server(services[0].secret.clone());
     
                crate::sam::services::lifx::sync(services[0].secret.clone());
                
            },
            Err(e) => {
                log::error!("{}", e);
            }
        }
    });
}



pub fn get_lifx_service_db_obj() -> Result<crate::sam::memory::Service, crate::sam::services::Error>{
    let mut pg_query = crate::sam::memory::PostgresQueries::default();
    pg_query.queries.push(crate::sam::memory::PGCol::String(format!("lifx")));
    pg_query.query_coulmns.push(format!("identifier ="));
    let service = crate::sam::memory::Service::select(None, None, None, Some(pg_query))?;
    return Ok(service[0].clone());
}

pub fn handle(_current_session: crate::sam::memory::WebSessions, request: &Request) -> Result<Response, crate::sam::http::Error> {
    if request.url() == "/api/services/lifx/list_all" {

        match get_lifx_service_db_obj(){
            Ok(service) => {
                let objects = crate::sam::services::lifx::get_all(service.secret.clone()).unwrap();
                return Ok(Response::json(&objects));
            },
            Err(e) => {
                log::error!("{}", e);
            }
        }

        return Ok(Response::empty_404());
    }

    if request.url() == "/api/services/lifx/public/list" {

        match get_lifx_service_db_obj(){
            Ok(service) => {
                let objects = crate::sam::services::lifx::get(service.secret.clone(), true).unwrap();
                return Ok(Response::json(&objects));
            },
            Err(e) => {
                log::error!("{}", e);
            }
        }

        return Ok(Response::empty_404());
    }

    if request.url() == "/api/services/lifx/private/list" {

        match get_lifx_service_db_obj(){
            Ok(service) => {
                let objects = crate::sam::services::lifx::get(service.secret.clone(), false).unwrap();
                return Ok(Response::json(&objects));
            },
            Err(e) => {
                log::error!("{}", e);
            }
        }

        return Ok(Response::empty_404());
    }


    if request.url() == "/api/services/lifx/set_state" {
        let input = post_input!(request, {
            selector: String,
            power: String,
            use_public: String,
        })?;

         



        match get_lifx_service_db_obj(){
            Ok(service) => {
          
                let mut public = false;
                if input.use_public == "true"{
                    public = true;
                }

                let objects = crate::sam::services::lifx::set(service.secret.clone(), input.selector.clone(), public, Some(input.power.clone()), None);


                
                return Ok(Response::json(&objects));
            },
            Err(e) => {
                log::error!("{}", e);
            }
        }




        return Ok(Response::empty_404());
    }

    if request.url() == "/api/services/lifx/set_color" {
        let input = post_input!(request, {
            selector: String,
            color: String,
            use_public: String
        })?;


        match get_lifx_service_db_obj(){
            Ok(service) => {
                let mut public = false;
                if input.use_public == "true"{
                    public = true;
                }
    
                crate::sam::services::lifx::set(service.secret.clone(), input.selector.clone(), public, None, Some(input.color.clone()));    
            },
            Err(e) => {
                log::error!("{}", e);
            }
        }


        return Ok(Response::empty_404());
    }

    
    return Ok(Response::empty_404());
}

pub fn get_lifx_endpoint() -> String {
    if check(Some(3)).is_ok(){
        return format!("https://api.lifx.com");
    } else {
        match get_lifx_service_db_obj(){
            Ok(service) => {
                return service.endpoint.clone();
            },
            Err(e) => {
                log::error!("{}", e);
            }
        }
    }
    return format!("https://api.lifx.com");
}

pub fn select_lifx_endpoint(public: bool) -> String {
    if public {
        return format!("https://api.lifx.com");
    } else {

        match get_lifx_service_db_obj(){
            Ok(service) => {
                return service.endpoint.clone();
            },
            Err(e) => {
                log::error!("{}", e);
            }
        }

        return format!("https://api.lifx.com");
    }
}

pub fn get_all(key: String) -> Result<lifx::Lights, crate::sam::services::Error>{
    let mut api_endpoints: Vec<String> = Vec::new();
    api_endpoints.push(get_lifx_endpoint());

    let config = lifx::LifxConfig{
        access_token: key.clone(),
        api_endpoints: api_endpoints
    };

    return Ok(lifx::Light::list_all(config.clone())?);
}

pub fn get(key: String, public: bool) -> Result<lifx::Lights, crate::sam::services::Error>{
    let mut api_endpoints: Vec<String> = Vec::new();
    api_endpoints.push(select_lifx_endpoint(public));

    let config = lifx::LifxConfig{
        access_token: key.clone(),
        api_endpoints: api_endpoints
    };

    return Ok(lifx::Light::list_all(config.clone())?);
}


pub fn set(key: String, selector: String, public: bool, power: Option<String>, color: Option<String>){
    let mut api_endpoints: Vec<String> = Vec::new();
    api_endpoints.push(select_lifx_endpoint(public));

    let lifx_config = lifx::LifxConfig{
        access_token: key.clone(),
        api_endpoints: api_endpoints
    };

    let mut state = lifx::State::new();
    state.power = power;
    state.color = color;

    // Turn off all lights
    match lifx::Light::set_state_by_selector(lifx_config.clone(), selector, state){
        Ok(_) => {},
        Err(e) => log::error!("failed to set lifx state: {:?}", e),
    }
}

pub fn set_state(key: String, selector: String, power: Option<String>, color: Option<String>){
    let mut api_endpoints: Vec<String> = Vec::new();
    api_endpoints.push(get_lifx_endpoint());

    let lifx_config = lifx::LifxConfig{
        access_token: key.clone(),
        api_endpoints: api_endpoints
    };

    let mut state = lifx::State::new();
    state.power = power;
    state.color = color;

    match lifx::Light::set_state_by_selector(lifx_config.clone(), selector, state){
        Ok(_) => {},
        Err(e) => log::error!("failed to set lifx state: {:?}", e),
    }
}


pub fn sync(key: String){

    let mut api_endpoints: Vec<String> = Vec::new();
    api_endpoints.push(format!("https://api.lifx.com"));

    let lifx_config = lifx::LifxConfig{
        access_token: key.clone(),
        api_endpoints: api_endpoints
    };

    let _storable_thing_vec: Vec<crate::sam::memory::Thing> = Vec::new();


    let lights = lifx::Light::list_all(lifx_config.clone()).unwrap();
    for light in lights{

        let mut thing = crate::sam::memory::Thing::new();

        // =================================================================
        // Sync Group/Location/Room/Name
        // =================================================================
        let location = light.location;
        let group = light.group;

        let mut loc = crate::sam::memory::Location::new();
        loc.name = location.name.clone();
        loc.save().unwrap();



        let mut pg_query = crate::sam::memory::PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(location.name.clone()));
        pg_query.query_coulmns.push(format!("name ilike"));

        let matching_locations = crate::sam::memory::Location::select(None, None, None, Some(pg_query)).unwrap();
        
        
        
        
        if matching_locations.len() > 0 {
            for matching_location in matching_locations{
                let mut room = crate::sam::memory::Room::new();
                room.name = group.name.clone();
                room.location_oid = matching_location.oid.clone();
                room.save().unwrap();
            }
        }

        // Get location oid
        let mut pg_query = crate::sam::memory::PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(location.name.clone()));
        pg_query.query_coulmns.push(format!("name ilike"));
        let locations = crate::sam::memory::Location::select(None, None, None, Some(pg_query)).unwrap();
        if locations.len() > 0 {
            let location_oid = locations[0].oid.clone();
              // Get room oid
              let mut pg_query = crate::sam::memory::PostgresQueries::default();
              pg_query.queries.push(crate::sam::memory::PGCol::String(location_oid.clone()));
              pg_query.query_coulmns.push(format!("location_oid ="));
              pg_query.queries.push(crate::sam::memory::PGCol::String(group.name.clone()));
              pg_query.query_coulmns.push(format!(" AND name ilike"));
            let rooms = crate::sam::memory::Room::select(None, None, None, Some(pg_query)).unwrap();
            if rooms.len() > 0 {
                thing.room_oid = rooms[0].oid.clone();
            }
        }

        // =================================================================
        // END Sync Group/Location/Room/Name
        // =================================================================

        let mut online_identifiers: Vec<String> = Vec::new();
        online_identifiers.push(light.id.clone());
        online_identifiers.push(light.uuid.clone());
        online_identifiers.push(light.label.clone());

       
        thing.name = light.label.clone();
        thing.thing_type = "lifx".to_string();
        thing.online_identifiers = online_identifiers.clone();
        
        let mut local_api_endpoints: Vec<String> = Vec::new();
        local_api_endpoints.push(get_lifx_endpoint());


        let local_config = lifx::LifxConfig{
            access_token: key.clone(),
            api_endpoints: local_api_endpoints
        };
        let xlocal_lights = lifx::Light::list_all(local_config.clone());
        match xlocal_lights{
            Ok(local_lights) => {
                for local_light in local_lights{
                    if local_light.label.clone() == light.label.clone() {
                        let mut local_identifiers: Vec<String> = Vec::new();
                        local_identifiers.push(local_light.id.clone());
                        local_identifiers.push(local_light.uuid.clone());
                        local_identifiers.push(local_light.label.clone());
                        thing.local_identifiers = local_identifiers.clone();
                    }
                }
            },
            Err(er) => {
                log::error!("{}", er);
            }
        }
        



        let existing_things = crate::sam::memory::Thing::select(None, None, None, None).unwrap();
        
        let mut already_exists = false;
        for existing_thing in existing_things{
            if existing_thing.name == light.label {
                already_exists = true;
            }

            for onlineid in thing.online_identifiers.clone(){
                for extonlineid in existing_thing.online_identifiers.clone(){
                    if onlineid == extonlineid{
                        already_exists = true;
                    }
                }
            }
        }

        if !already_exists{
            thing.save().unwrap();
        }

    }

    sync_local(key.clone());
    sync_local(key);
}



pub fn sync_local(key: String){
    let mut api_endpoints: Vec<String> = Vec::new();
    api_endpoints.push(get_lifx_endpoint());

    let lifx_config = lifx::LifxConfig{
        access_token: key.clone(),
        api_endpoints: api_endpoints
    };

    let _storable_thing_vec: Vec<crate::sam::memory::Thing> = Vec::new();

    let lights = lifx::Light::list_all(lifx_config.clone()).unwrap();
    for light in lights{

        let mut local_identifiers: Vec<String> = Vec::new();
        local_identifiers.push(light.id.clone());
        local_identifiers.push(light.uuid.clone());
        local_identifiers.push(light.label.clone());

        let mut thing = crate::sam::memory::Thing::new();
        thing.name = light.label.clone();
        thing.thing_type = "lifx".to_string();
        thing.local_identifiers = local_identifiers.clone();
        


        let existing_things = crate::sam::memory::Thing::select(None, None, None, None).unwrap();
        
        let mut already_exists = false;
        for existing_thing in existing_things{
            if existing_thing.name == light.label {
                already_exists = true;
            }

            for onlineid in thing.local_identifiers.clone(){
                for extonlineid in existing_thing.local_identifiers.clone(){
                    if onlineid == extonlineid{
                        already_exists = true;
                    }
                }
            }
        }

        if !already_exists{
            thing.save().unwrap();
        }

    }
}