use std::{fs::File, io::{BufReader, Write}};

use rodio::Decoder;
use serde::{Deserialize, Serialize};

/*
example
{
  "volumeScale": 1.0,
  "pitchScale": 1.0,
  "intonationScale": 1.0,
  "prePhonemeLength": 0.1,
  "postPhonemeLength": 0.5,
  "outputSamplingRate": 24000,
  "text":"h",
  "processingAlgorithm": "string",
  "styleId": 1403759395,
  "speakerUuid":"9bf2ab50-c756-11ec-9374-0242ac1c0002",
  "prosodyDetail": [],
  "speedScale": 1.0
}
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct synth_info{
    pub speakerUuid:String,
    pub styleId:i32,
    pub text:String,
    pub speedScale:f32,
    pub volumeScale:f32,
    pub prosodyDetail:Vec<i32>,
    pub pitchScale:f32,
    pub intonationScale:f32,
    pub prePhonemeLength:f32,
    pub postPhonemeLength:f32,
    pub outputSamplingRate:i32,
}

pub async fn synth(text:String,)
{
    let info = synth_info{
        volumeScale: 1.0,
        pitchScale: 0.0,
        intonationScale: 1.0,
        prePhonemeLength: 0.1,
        postPhonemeLength: 0.5,
        outputSamplingRate: 24000,
        text:text,
        styleId: 1403759397,
        speakerUuid:"9bf2ab50-c756-11ec-9374-0242ac1c0002".to_string(),
        prosodyDetail: vec![],
        speedScale: 1.0
    };
    let url = "http://localhost:50032/v1/synthesis";
    let result = reqwest::Client::new()
        .post(url).json(&info)
        .send().await;
    match result{
        Ok(response)=>{
            let stream_handle = rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
            let sink = rodio::Sink::connect_new(&stream_handle.mixer());
            let mut file = File::create("audio.wav").expect("Failed to open file");
            let _ = file.write_all(&response.bytes().await.unwrap());
            let buf = BufReader::new(File::open("audio.wav").unwrap());
            let source = Decoder::new(buf).unwrap();
            sink.append(source);
            sink.sleep_until_end();
            return;
        }
        Err(_)=>{
            println!("Please start COEIROINK");
        }
    }
}