use tokio::time::Instant;

use crate::{access_bible::BibleInfo, coeiroink::synth, network::create_url_from_bible_info};

use std::env;

pub mod access_bible;
pub mod network;
pub mod coeiroink;

#[tokio::main] 
async fn main() {
    let mut not_found_count = 0;
    let args:Vec<String> = env::args().collect();
    
    if args.len() == 0
    {
        println!("No arguments were passed on the command line. To execute the command, please specify the following arguments: <book>,<chapter>,<verses>,<version>.");
        return;
    }else{
        if args.len() > 5
        {
            println!("Too many arguments!To execute the command, please specify the following arguments: <book>,<chapter>,<verses>,<version>.");
            return;
        }
    }

    let mut n_verse = 1;
    let mut n_chapter = 1;
    for _ in 1..150{
        /*
        book:"John".to_string(),
        chapter:n_chapter.to_string(),
        verses:n_verse.to_string(),
        version:"4016".to_string(),
         */
        let info = BibleInfo{
            book:args[1].to_string(),
            chapter:args[2].to_string(),
            verses:args[3].to_string(),
            version:args[4].to_string(),
        };

        let mut time = Instant::now();
        let text = create_url_from_bible_info("localhost:3000".to_string(),info,&mut not_found_count).await;
        match text
        {
            Ok(result) => {
                time = Instant::now();
                synth(result).await;
                n_verse += 1;
            },
            Err(result) => {
                if result{
                    n_verse = 1;
                    n_chapter += 1;
                    if (time - Instant::now()).as_secs() != 0{
                        not_found_count = 0;
                    }
                }else{
                    println!("Launch YouVersion-API on localhost. https://github.com/Glowstudent777/YouVersion-API");
                }
            },
        }
        println!("The maximum number of readings has been reached. Chapter {}, Verse {} has been read.",n_chapter,n_verse);
    }
}