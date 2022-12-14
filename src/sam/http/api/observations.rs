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

use std::path::Path;

pub fn handle(_current_session: crate::sam::memory::WebSessions, request: &Request) -> Result<Response, crate::sam::http::Error> {
    if request.url() == "/api/observations" {
        let skip = request.get_param("skip");
        let mut skip_number: usize = 0;
        if skip.is_some(){
            skip_number = skip.unwrap().parse::<usize>().unwrap();
        }

        let objects = crate::sam::memory::Observation::select_lite(Some(1), Some(skip_number), Some(format!("timestamp DESC")), None)?;
        return Ok(Response::json(&objects));
    }


    if request.url().contains("/api/observations/file/") {
        let url = request.url();
        let split = url.split("/");
        let vec: Vec<&str> = split.collect();
        let oid = vec[4];

        // Build query
        let mut pg_query = crate::sam::memory::PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(oid.clone().to_string()));
        pg_query.query_coulmns.push(format!("oid ="));

        // Select project by oid 
        let observations = crate::sam::memory::Observation::select(None, None, None, Some(pg_query)).unwrap();
        let observation = observations[0].clone();

        let response = Response::from_data("audio/wav", observation.observation_file.unwrap());

        return Ok(response);
    }

    // Visual Wav Builder
    if request.url().contains("/api/observations/vwav/") {
        let url = request.url();
        let split = url.split("/");
        let vec: Vec<&str> = split.collect();
        let oid = vec[4];

        // Build query
        let mut pg_query = crate::sam::memory::PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(oid.clone().to_string()));
        pg_query.query_coulmns.push(format!("oid ="));

        // Select project by oid 
        let observations = crate::sam::memory::Observation::select(None, None, None, Some(pg_query)).unwrap();
        let observation = observations[0].clone();

        let wav_data = observation.observation_file.unwrap();

        let tmp_file_path = format!("/opt/sam/tmp/observations/vwav/{}.wav", observation.oid);

        // Use cached tmp file if it already exists
        let cache_path = format!("{}.16.wav.mp4", tmp_file_path.clone());
        if Path::new(&cache_path).exists(){
            let data = std::fs::read(format!("{}.16.wav.mp4", tmp_file_path.clone()).as_str())?;
            let response = Response::from_data("video/mp4", data);
            return Ok(response);
        }


        std::fs::write(tmp_file_path.clone(), wav_data)?;

        crate::sam::tools::linux_cmd(format!("ffmpeg -i {} -ar 16000 -ac 1 -c:a pcm_s16le {}.16.wav", tmp_file_path.clone(), tmp_file_path.clone()));

        crate::sam::tools::linux_cmd(format!("/opt/sam/bin/whisper -m /opt/sam/models/ggml-base.en.bin -f {}.16.wav -owts", tmp_file_path.clone()));
    
        crate::sam::services::stt::patch_whisper_wts(format!("{}.16.wav.wts", tmp_file_path.clone()))?;

        crate::sam::tools::linux_cmd(format!("chmod +x {}.16.wav.wts", tmp_file_path.clone()));

        crate::sam::tools::linux_cmd(format!("{}.16.wav.wts", tmp_file_path.clone()));

        let data = std::fs::read(format!("{}.16.wav.mp4", tmp_file_path.clone()).as_str())?;

        let response = Response::from_data("video/mp4", data);

        // Cleanup
        crate::sam::tools::linux_cmd(format!("rm {}", tmp_file_path.clone()));
        crate::sam::tools::linux_cmd(format!("rm {}.16.wav", tmp_file_path.clone()));
        crate::sam::tools::linux_cmd(format!("rm {}.16.wav.wts", tmp_file_path.clone()));
        // crate::sam::tools::linux_cmd(format!("rm {}.16.wav.mp4", tmp_file_path.clone()));

        return Ok(response);
    }



    return Ok(Response::empty_404());
}