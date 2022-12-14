// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

use std::fs;
use std::io;
use std::path::{Path};
use std::process::{Command, Stdio};
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        Postgres(postgres::Error);
        Hound(hound::Error);
    }
}

pub fn python3(command: String) -> String{
    let cmd = Command::new("python3")
    .arg(command.clone())
    .output()
    .unwrap();
    return String::from_utf8_lossy(&cmd.stdout).to_string();
}

pub fn idfk(command: &str) -> String {
    let child = Command::new("/bin/python3")
    .arg(command)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");

    let output = child
        .wait_with_output()
        .expect("failed to wait on child");

    return String::from_utf8_lossy(&output.stdout).to_string();
}

pub fn pip3(command: &str) -> String {
    let child = Command::new("/bin/pip3")
    .arg(command)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");

    let output = child
        .wait_with_output()
        .expect("failed to wait on child");

    return String::from_utf8_lossy(&output.stdout).to_string();
}

pub fn cmd(command: String) -> String{
    let cmd = Command::new("sh")
    .arg("-c")
    .arg(command.clone())
    .output()
    .unwrap();
    return String::from_utf8_lossy(&cmd.stdout).to_string();
}


pub fn linux_cmd(command: String){
    let cmd = Command::new("sh")
    .arg("-c")
    .arg(command.clone())
    .output()
    .expect("failed to execute process");
    if cmd.status.success() {
        let er = String::from_utf8_lossy(&cmd.stdout);
        log::info!("{}:{}",command,  er);
    } else {
        let er = String::from_utf8_lossy(&cmd.stderr);
        log::error!("{}:{}",command,  er);
    }
}

pub fn does_wav_have_sounds(audio_filename: String) -> Result<bool>{
    let mut has_sounds = false;
	let threshold = 14000 as i16;

	let mut audio_file = hound::WavReader::open(audio_filename)?;

	let raw_samples = audio_file.samples::<i16>().into_iter().map(|x| x.unwrap()).collect::<Vec<i16>>();

	let mut samples: Vec<i16> = Vec::new();

	for i in 0..=raw_samples.len() - 1 {
		if i % 100 == 0 {
	

			if raw_samples[i as usize] > threshold || raw_samples[i as usize] < -threshold{
				has_sounds = true;
			}

			samples.push(raw_samples[i as usize]);
		}
	}

    return Ok(has_sounds);
}


pub fn extract_zip(zip_path: &str, extract_path: String) -> i32 {

    let fname = std::path::Path::new(zip_path);
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath_end = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        let out_mend = extract_path.to_owned() + outpath_end.to_str().unwrap();

        let outpath = Path::new(&(out_mend));

        {
            let comment = file.comment();
            if !comment.is_empty() {
                // log::info!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            log::info!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            log::info!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    0
}