// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

use simple_websockets::{Event, Responder};
use std::collections::HashMap;
use std::thread;

pub fn init() {
    thread::spawn(move || {
        // listen for WebSockets on port 2794:
        let event_hub = simple_websockets::launch(2794)
            .expect("failed to listen on port 2794");
        // map between client ids and the client's `Responder`:
        let mut clients: HashMap<u64, Responder> = HashMap::new();

        loop {
            match event_hub.poll_event() {
                Event::Connect(client_id, responder) => {
                    log::info!("A WSS client connected with id #{}", client_id);
                    // add their Responder to our `clients` map:
                    clients.insert(client_id, responder);
                },
                Event::Disconnect(client_id) => {
                    log::info!("WSS Client #{} disconnected.", client_id);
                    // remove the disconnected client from the clients map:
                    clients.remove(&client_id);
                },
                Event::Message(client_id, message) => {
                    log::info!("WSS Received a message from client #{}: {:?}", client_id, message);
                    // retrieve this client's `Responder`:
                    let responder = clients.get(&client_id).unwrap();
                    // echo the message back:
                    responder.send(message);
                },
            }
        }
    });
}