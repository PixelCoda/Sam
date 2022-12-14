// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.




use std::thread;
use std::path::Path;

// TO



pub fn init(){
    // Initialize RTSP Cameras
    // TODO - Customizable Port and Path
    thread::spawn(move || {
        let mut pg_query = crate::sam::memory::PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(format!("rtsp")));
        pg_query.query_coulmns.push(format!("thing_type ="));
        let rtsp_things = crate::sam::memory::Thing::select(None, None, None, Some(pg_query));

        match rtsp_things {
            Ok(things) => {
   
     
                for thing in things{

                    // Convert RTSP to /streams http api
                    let rtsp_http_thing = thing.clone();
                    thread::spawn(move || {
                        let rtsp_address = format!("rtsp://{}:{}@{}:554/cam/realmonitor?channel=1&subtype=0", rtsp_http_thing.username, rtsp_http_thing.password, rtsp_http_thing.ip_address);
                        let script = crate::sam::services::rtsp::gen_rtsp_to_http_stream_script(rtsp_address, rtsp_http_thing.oid);
                        crate::sam::tools::linux_cmd(script);
                    });


                    // Convert RTSP streams to wav files for sam to parse
                    let rtsp_wav_thing = thing.clone();
                    thread::spawn(move || {
                        let rtsp_address = format!("rtsp://{}:{}@{}:554/cam/realmonitor?channel=1&subtype=0", rtsp_wav_thing.username, rtsp_wav_thing.password, rtsp_wav_thing.ip_address);
                        let script = crate::sam::services::rtsp::gen_rtsp_to_wav_script(rtsp_address, rtsp_wav_thing.oid);
                        crate::sam::tools::linux_cmd(script);
                    });


          
                    

                    // TODO - Perform Deep Learning on RTSP streams and log observations

                    // TODO - Record slected RTSP streams to a network location
                }
                
            },
            Err(e) => {
                log::error!("{}", e);
            }
        }
    });
}

pub fn gen_rtsp_to_http_stream_script(address: String, identifier: String) -> String{
    let mut script = format!("#!/bin/bash\n");
    script = format!("{}VIDSOURCE=\"{}\"\n", script, address);
    script = format!("{}AUDIO_OPTS=\"-c:a aac -b:a 160000 -ac 2\"\n", script);
    script = format!("{}VIDEO_OPTS=\"-s 854x480 -c:v libx264 -b:v 800000\"\n", script);
    script = format!("{}OUTPUT_HLS=\"-hls_time 10 -hls_list_size 10 -start_number 1\"\n", script);
    script = format!("{}ffmpeg -i \"$VIDSOURCE\" -y $AUDIO_OPTS $VIDEO_OPTS $OUTPUT_HLS /opt/sam/streams/{}.m3u8", script, identifier);
    return script;
}

pub fn gen_rtsp_to_wav_script(address: String, identifier: String) -> String{
    let p = format!("/opt/sam/tmp/sound/{}", identifier);
    if !Path::new(p.as_str()).exists(){
        crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/tmp/sound/{}", identifier));
        crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/tmp/sound/{}/s1", identifier));
        crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/tmp/sound/{}/s2", identifier));
        crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/tmp/sound/{}/s3", identifier));
    }


    let mut script = format!("#!/bin/bash\n");
    script = format!("{}VIDSOURCE=\"{}\"\n", script, address);
    script = format!("{}ffmpeg -i \"$VIDSOURCE\" -f segment -segment_time 1 -reset_timestamps 1 -strftime 1 -map 0:a /opt/sam/tmp/sound/{}/s1/%Y%m%d-%H%M%S.wav", script, identifier);
    return script;
}



