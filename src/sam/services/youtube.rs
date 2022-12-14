use rouille::Request;
use rouille::Response;
use serde::{Serialize, Deserialize};


use invidious::reqwest::blocking::Client;


pub fn handle(current_session: crate::sam::memory::WebSessions, request: &Request) -> Result<Response, crate::sam::http::Error> {
    if request.url() == "/api/services/youtube" {

        let q_param = request.get_param("q");

        match q_param {
            Some(q) => {
                let client = Client::new(String::from("https://vid.puffyan.us"));
                let search_results = client.search(Some(format!("q={}", q).as_str())).unwrap().items;
                return Ok(Response::json(&search_results));
            },
            None => {
                return Ok(Response::empty_404());
            }

        }
    }

    if request.url() == "/api/services/youtube/lucky" {

        let q_param = request.get_param("q");

        match q_param {
            Some(q) => {
                let client = Client::new(String::from("https://vid.puffyan.us"));
                let search_results = client.search(Some(format!("q={}", q).as_str())).unwrap().items;
                let video = search_results[0].clone();
                return Ok(Response::json(&video));
            },
            None => {
                return Ok(Response::empty_404());
            }

        }
    }

    if request.url() == "/api/services/youtube/stream" {

        let id_param = request.get_param("id");
        match id_param {
            Some(id) => {
                let url = format!("https://youtu.be/{}", id);
                let path_to_video = rustube::blocking::download_worst_quality(url.as_str())?;
                log::info!("path_to_video: {:?}", path_to_video);
                let data = std::fs::read(path_to_video).expect("Unable to read file");

                let response = Response::from_data("video/mp4", data);
                return Ok(response);

            },
            None => {
                return Ok(Response::empty_404());
            }

        }
    }


    if request.url() == "/api/services/youtube/download" {

        let id = request.get_param("id").unwrap();

        let tube_id = rustube::Id::from_string(id)?;
        let video = rustube::blocking::Video::from_id(tube_id.clone())?;

        log::info!("video: {:?}", video);

        let best_quality = video
            .streams()
            .iter()
            .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
            .max_by_key(|stream| stream.quality_label).unwrap();


        best_quality.blocking_download_to_dir("/opt/sam/tmp/youtube/downloads")?;

        let data = std::fs::read(format!("/opt/sam/tmp/youtube/downloads/{}.mp4", tube_id.clone())).expect("Unable to read file");


        let mut file_folder_tree: Vec<String> = Vec::new();
        file_folder_tree.push(format!("Videos"));
        file_folder_tree.push(format!("Youtube"));

        let mut file = crate::sam::memory::FileStorage::new();
        file.file_name = format!("{}.mp4", tube_id.clone());
        file.file_type = format!("video/mp4");
        file.file_data = Some(data);
        file.file_folder_tree = Some(file_folder_tree);
        file.storage_location_oid = format!("SQL");
        file.save()?;


        let mut notify = crate::sam::memory::Notification::new();
        notify.message = format!("{}.mp4 finished downloading!", tube_id.clone());
        notify.human_oid = current_session.human_oid;
        notify.sid = current_session.sid;
        notify.save()?;

        let response = Response::text("done");
        return Ok(response);
    }


    if request.url() == "/api/services/youtube/cache" {

        let id_param = request.get_param("id");
        match id_param {
            Some(id) => {
                

                let tube_id = rustube::Id::from_string(id)?;
                let video = rustube::blocking::Video::from_id(tube_id)?;

                let best_quality = video
                    .streams()
                    .iter()
                    .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
                    .min_by_key(|stream| stream.quality_label).unwrap();


                best_quality.blocking_download_to_dir("/opt/sam/tmp/youtube")?;


                return Ok(Response::text("done"));

            },
            None => {
                return Ok(Response::empty_404());
            }

        }
    }

    return Ok(Response::empty_404());
}