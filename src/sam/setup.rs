// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

use serde::{Serialize, Deserialize};
use error_chain::error_chain;
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        Postgres(postgres::Error);
        Hound(hound::Error);
    }
}

// TODO - Install CUDA
// TODO - Install Snapcast Server
// TODO - Build rust library for communication with Snapcast API
// TODO - cargo install librespot
// cp /home/kal/.cargo/bin/librespot /bin/librespot
pub async fn install() {

    match update().await{
        Ok(_) => {
            log::info!("Successfully updated!");
        },
        Err(e) => {
            log::error!("Failed to update! {:?}", e);
        }
    }

    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam"));
    crate::sam::tools::linux_cmd("chmod -R 777 /opt/sam".to_string());
    crate::sam::tools::linux_cmd("chown 1000 -R /opt/sam".to_string());
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/bin"));
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/dat"));
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/streams"));
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/models"));
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/files"));
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/scripts"));
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/scripts/rivescript"));
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/scripts/who.io"));
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/scripts/who.io/dataset"));
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/scripts/sprec"));
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/scripts/sprec/audio"));
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/scripts/sprec/noise"));
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/scripts/sprec/noise/_background_noise_"));
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/scripts/sprec/noise/other"));
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/tmp"));
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/tmp/youtube"));
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/tmp/youtube/downloads"));
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/tmp/sound"));
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/tmp/observations"));
    crate::sam::tools::linux_cmd(format!("mkdir /opt/sam/tmp/observations/vwav"));
    match crate::sam::services::darknet::install(){
        Ok(_) => {
            log::info!("darknet installed successfully");
        },
        Err(e) => {
            log::error!("Failed to install darknet: {}", e);
        }
    }

    match crate::sam::services::sprec::install(){
        Ok(_) => {
            log::info!("sprec installed successfully");
        },
        Err(e) => {
            log::error!("Failed to install sprec: {}", e);
        }
    }

    match crate::sam::services::rivescript::install(){
        Ok(_) => {
            log::info!("rivescript installed successfully");
        },
        Err(e) => {
            log::error!("Failed to install rivescript: {}", e);
        }
    }

    match crate::sam::services::who::install(){
        Ok(_) => {
            log::info!("who.io installed successfully");
        },
        Err(e) => {
            log::error!("Failed to install who.io: {}", e);
        }
    }

    match crate::sam::services::snapcast::install(){
        Ok(_) => {
            log::info!("Snapcast server installed successfully");
        },
        Err(e) => {
            log::error!("Failed to install snapcast server: {}", e);
        }
    }

    match crate::sam::services::stt::install(){
        Ok(_) => {
            log::info!("STT server installed successfully");
        },
        Err(e) => {
            log::error!("Failed to install STT server: {}", e);
        }
    }

    match crate::sam::services::image::install(){
        Ok(_) => {
            log::info!("Image service installed successfully");
        },
        Err(e) => {
            log::error!("Failed to install image service: {}", e);
        }
    }


    #[cfg(not(debug_assertions))]{
        match crate::sam::http::install(){
            Ok(_) => {
                log::info!("HTTP server installed successfully");
            },
            Err(e) => {
                log::error!("Failed to install HTTP server: {}", e);
            }
        }
    }
    
}


// Check: https://osf.opensam.foundation/api/packages for updates
pub async fn update() -> Result<()>{
   
    let request = reqwest::Client::new().get("https://osf.opensam.foundation/api/packages").send().await?;
    let packages = request.json::<Packages>().await?;
    
    for package in packages{
        if package.latest_version != crate::VERSION.ok_or("0.0.0")? && package.name == "sam"{
            log::warn!("UPDATE_CHECK: S.A.M. needs an update");
        } 
    }

    return Ok(());

}



pub type Packages = Vec<Package>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub name: String,
    pub versions: Vec<String>,
    #[serde(rename = "latest_version")]
    pub latest_version: String,
    #[serde(rename = "latest_oid")]
    pub latest_oid: String,
}


pub fn uninstall(){

}

