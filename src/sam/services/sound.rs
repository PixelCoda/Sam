// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.



use dasp::Frame;
use hound::{WavReader, WavSpec, WavWriter};
use noise_gate::NoiseGate;
use std::{
    fs::{File},
    io::BufWriter,
    path::PathBuf,
    path::Path,
    thread,
    time::{SystemTime, UNIX_EPOCH}
};


pub fn init(){
    crate::sam::services::sound::s2_init();
    crate::sam::services::sound::s3_init();

}

// TODO - Send hot sound observation to sam before storing in SQL database
pub fn observe(prediction: crate::sam::services::stt::STTPrediction, file_path: &str){
    let mut observation = crate::sam::memory::Observation::new();
    observation.observation_type = crate::sam::memory::ObservationType::HEARD;
    observation.observation_notes.push(prediction.stt.clone());

    let data = std::fs::read(file_path.clone()).unwrap();
    observation.observation_file = Some(data);


    log::info!("file_path: {:?}",file_path.clone());

    let mut split = file_path.split("/");
    let vec = split.collect::<Vec<&str>>();

    log::info!("file_path_vec: {:?}",vec.clone());

    if prediction.stt.len() > 0 {
        observation.observation_objects.push(crate::sam::memory::ObservationObjects::PERSON);
    }

    if prediction.human.contains("Unknown"){
        let mut human = crate::sam::memory::Human::new();
        human.name = prediction.human;
        human.heard_count = 1;
        human.save().unwrap();

        observation.observation_humans.push(human);
    } else {
        let mut pg_query = crate::sam::memory::PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(prediction.human.clone()));
        pg_query.query_coulmns.push(format!("oid ilike"));
        let humans = crate::sam::memory::Human::select(None, None, None, Some(pg_query)).unwrap();
        if humans.len() > 0{
            observation.observation_humans.push(humans[0].clone());
        } else {
            let mut human = crate::sam::memory::Human::new();
            human.name = format!("Unknown");
            human.heard_count = 1;
            human.save().unwrap();
            observation.observation_humans.push(human);
        }
    }


    // observation.observation_humans

    observation.save().unwrap();
}


// Stage One - Destroys samples that fall below the noise threshold
pub fn s1_init(){
    // thread::spawn(move || {
    //     loop {
    //         thread::sleep(std::time::Duration::from_millis(1000));
    //         let paths = std::fs::read_dir("/opt/sam/tmp/sound/s1").unwrap();

    //         for path in paths {
    //             let spath = path.unwrap().path().display().to_string();
    //             let has_sounds = crate::sam::tools::does_wav_have_sounds(spath.clone());
    //             match has_sounds {
    //                 Ok(has_sounds) => {
    //                     if!has_sounds{
    //                         std::fs::remove_file(spath).unwrap();
    //                     }
    //                 },
    //                 Err(e) => {
    //                     log::error!("{}", e);
    //                 }

    //             }
              
    //         }
    //     }
    // });
}

// Stage Two - Destroys samples that don't reach the noise threshold and trims "whitespace"
// Results are stored in /opt/sam/tmp/sound/s2
pub fn s2_init() {
    thread::spawn(move || {
        loop {


            let thing_paths = std::fs::read_dir("/opt/sam/tmp/sound").unwrap();
            for thing_path in thing_paths{
                let tpath = thing_path.unwrap().path().display().to_string();



                let paths = std::fs::read_dir(format!("{}/s1", tpath).as_str()).unwrap();

                for path in paths {
                    let spath = path.unwrap().path().display().to_string();
    
                    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    
                    // open the WAV file
                    let xreader = WavReader::open(&spath.clone());
    
                    match xreader{
                        Ok(reader) => {
    
                            // we need the header to determine the sample rate
                            let header = reader.spec();
                            // read all the samples into memory, converting them to a single-channel
                            // audio stream
                            let xsamples = reader
                                .into_samples::<i16>()
                                .map(|result| result.map(|sample| [sample]))
                                .collect::<Result<Vec<_>, _>>();

                            match xsamples{
                                Ok(samples) => {
                                    let release_time = (header.sample_rate as f32 * 0.3).round();
    
                                    // make sure the output directory exists
                            
                                    // initialize our sink
            
                                    let s2_path = PathBuf::from(format!("{}/s2", tpath).as_str());
            
            
                                    let mut sink = Sink::new(s2_path, format!("{}-", timestamp), header);
            
                                    // set up the NoiseGate
                                    let mut gate = NoiseGate::new(14000, release_time as usize);
                                    // and process all the samples
                                    gate.process_frames(&samples, &mut sink);
            
                                    match std::fs::remove_file(spath){
                                        Ok(_) => {},
                                        Err(err) => {
                                            log::error!("{}", err);
                                        }
                                    }
                                   
                                },
                                Err(err) => {
                                    log::error!("{}", err);
                                }
                            }
    
                          
    
                        },
                        Err(e) => {
                            let es = e.to_string();
                            if !es.contains("read enough bytes"){
                                log::error!("{}", e);
                            }
                        }
                    }
    
    
                   
    
        
                }




            }




 
        }
    });


}

// Stage Three - Stitches files into a single timestamped file ready to be observed by SAM
// Results are stored in /opt/sam/tmp/sound/s3
pub fn s3_init() {
    thread::spawn(move || {
        loop {

            let thing_paths = std::fs::read_dir("/opt/sam/tmp/sound").unwrap();
            for thing_path in thing_paths{
                let tpath = thing_path.unwrap().path().display().to_string();


                let current_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
                let paths = std::fs::read_dir(format!("{}/s2", tpath).as_str()).unwrap();
    
    
                let mut timestamps_to_stitch: Vec<i64> = Vec::new();
                let mut paths_to_stitch: Vec<String> = Vec::new();
    
                for path in paths {
                    let spath = path.unwrap().path().display().to_string();
    
                    let file_name = spath.clone().replace(format!("{}/s2/", tpath).as_str(), "").replace(".wav", "");
    
                    // log::info!("file_name: {:?}", file_name);
                    let split = file_name.split("-");
                    let vec = split.collect::<Vec<&str>>();
    
                    let file_timestamp = vec[0].to_string().parse::<i64>().unwrap();
                    let file_id = vec[1].to_string().parse::<i64>().unwrap();
    
                    if timestamps_to_stitch.len() == 0 {
                        timestamps_to_stitch.push(file_timestamp.clone());
                    } else {
                        for stamp in timestamps_to_stitch.clone(){
                            if file_timestamp == (stamp - 1) || file_timestamp == (stamp + 1){
                                timestamps_to_stitch.push(file_timestamp.clone());
                            }
                        }
                    }
                    
    
                }

                let paths = std::fs::read_dir(format!("{}/s2", tpath).as_str()).unwrap();
                for path in paths {
                    let spath = path.unwrap().path().display().to_string();

                    let file_name = spath.clone().replace(format!("{}/s2/", tpath).as_str(), "").replace(".wav", "");
    
                    let split = file_name.split("-");
                    let vec = split.collect::<Vec<&str>>();
    
                    let file_timestamp = vec[0].to_string().parse::<i64>().unwrap();
                    let file_id = vec[1].to_string().parse::<i64>().unwrap();
    
                    if timestamps_to_stitch.contains(&file_timestamp){
                        paths_to_stitch.push(spath.clone());
                    }
                }
    
    
                timestamps_to_stitch.sort_by(|a, b| a.partial_cmp(b).unwrap());
                paths_to_stitch.sort_by(|a, b| a.partial_cmp(b).unwrap());

                // log::info!("timestamps_to_stitch: {:?}", timestamps_to_stitch);
                // log::info!("paths_to_stitch: {:?}", paths_to_stitch);
    
                let mut should_abort = false;
                for xstamp in timestamps_to_stitch.clone(){
                    if xstamp >= current_timestamp {
                        should_abort = true;
                        break;
                    }
                    if xstamp >= current_timestamp-1 {
                        should_abort = true;
                        break;
                    }
                }

                if timestamps_to_stitch.len() == 0{
                    should_abort = true;
                }
    
                if !should_abort {

        
                    for path_to_stitch in paths_to_stitch{
                        // open the WAV file
                        let xreader = WavReader::open(&path_to_stitch.clone());
        
                        match xreader{
                            Ok(reader) => {
        
                                // we need the header to determine the sample rate
                                let header = reader.spec();
        
        
                                let samples = reader
                                .into_samples::<i16>()
                                .map(|result| result.map(|sample| [sample]))
                                .collect::<Result<Vec<_>, _>>().unwrap();
        
        
                                let outspec = hound::WavSpec {
                                    channels: header.channels,
                                    sample_rate: header.sample_rate,
                                    bits_per_sample: header.bits_per_sample,
                                    sample_format: hound::SampleFormat::Int,
                                };
                                
                                let fpath = format!("{}/s3/{}.wav",tpath, timestamps_to_stitch[0].clone());
                                let ffpath = fpath.clone();
                                let outpath: &Path = Path::new(&ffpath);
                            
                                let mut writer = match outpath.is_file() {
                                    true => hound::WavWriter::append(outpath).unwrap(),
                                    false => hound::WavWriter::create(outpath, outspec).unwrap(),
                                };
        
                                for sample in samples{
                                    writer.write_sample(sample[0]).unwrap();
                                }
                  
                                writer.finalize().unwrap();


                                std::fs::remove_file(path_to_stitch.clone()).unwrap();
        
        
                            },
                            Err(e) => {
                                let es = e.to_string();
                                if !es.contains("read enough bytes"){
                                    log::error!("{}", e);
                                }
                            }
                        }
                    }

                    let fpath = format!("{}/s3/{}.wav",tpath, timestamps_to_stitch[0].clone());

                    // Build STT prediction
                    let stt = crate::sam::services::stt::process(fpath.clone()).unwrap();

                    if stt.stt.len() > 0 {
                        // Hot Ding
                        crate::sam::tools::linux_cmd(format!("aplay /opt/sam/beep.wav"));


                        // Observe Sound + Prediction
                        observe(stt, &fpath.clone());
                    }

          

                    std::fs::remove_file(fpath.clone()).unwrap();
                }
    
            }






         




        }
    });
}

pub struct Sink {
    output_dir: PathBuf,
    clip_number: usize,
    prefix: String,
    spec: WavSpec,
    writer: Option<WavWriter<BufWriter<File>>>,
}

impl Sink {
    pub fn new(output_dir: PathBuf, prefix: String, spec: WavSpec) -> Self {
        Sink {
            output_dir,
            prefix,
            spec,
            clip_number: 0,
            writer: None,
        }
    }

    fn get_writer(&mut self) -> &mut WavWriter<BufWriter<File>> {
        if self.writer.is_none() {
            // Lazily initialize the writer. This lets us drop the writer when 
            // sent an end_of_transmission and have it automatically start
            // writing to a new clip when necessary.
            let filename = self
                .output_dir
                .join(format!("{}{}.wav", self.prefix, self.clip_number));
            self.clip_number += 1;
            self.writer = Some(WavWriter::create(filename, self.spec).unwrap());
        }

        self.writer.as_mut().unwrap()
    }
}

impl<F> noise_gate::Sink<F> for Sink
where
    F: Frame,
    F::Sample: hound::Sample,
{
    fn record(&mut self, frame: F) {
        let writer = self.get_writer();

        // write all the channels as interlaced audio
        for channel in frame.channels() {
            writer.write_sample(channel).unwrap();
        }
    }

    fn end_of_transmission(&mut self) {
        // if we were previously recording a transmission, remove the writer
        // and let it flush to disk
        if let Some(writer) = self.writer.take() {
            writer.finalize().unwrap();
        }
    }
}
