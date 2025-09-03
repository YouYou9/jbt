use serde::{Deserialize, Serialize};
use serde_json;
use crate::access_bible::{BibleInfo};
#[derive(Debug, Serialize, Deserialize)]
struct Verse {
    citation:String,
    passage:String,
}
#[derive(Debug, Serialize, Deserialize)]
struct VerseError{
    code:i32,
    message:String,
}
//次の章に行く場合、ErrとしてTrueとして返します。接続エラーの場合はfalseをErrとして返します。
pub async fn create_url_from_bible_info(server_dns:String,info:BibleInfo,not_found_count:&mut i32)->Result<String,bool>{
    let url = format!("http://{}/api/v1/verse?book={}&chapter={}&verses={}&version={}",server_dns,info.book,info.chapter,info.verses,info.version);
    let result = reqwest::Client::new()
        .get(url)
        .send().await;
    match result{
        //接続が確保されたら。
        Ok(fetch_result)=>
        {
            //所有権問題でここでbytesを一旦分解。
            let bytes = fetch_result.bytes().await.unwrap();
            //reqwestは、serde_jsonでjsonを処理している。
            let into_convert:Result<Verse, _> = serde_json::from_slice(&bytes);//Infallibleです。
            match into_convert{
                Ok(verse)=>{
                    Ok(verse.passage)
                },
                Err(_)=>{
                    //serde_json側で処理させれば所有権問題解決
                    //Verseでintoできなければ
                    let verse_error:Result<VerseError,_> = serde_json::from_slice(&bytes);//Infallibleです。
                    match verse_error
                    {
                        Ok(n)=>{
                            *not_found_count += 1;
                            if n.message != "Verse not found" || *not_found_count >= 2
                            {
                                std::process::exit(0);
                            }
                            return Err(true)
                        },
                        Err(_)=>{
                            println!("The program was unable to parse the API response. An unknown error was returned from the API. Result{}",String::from_utf8_lossy(&bytes).to_owned());
                            return Err(false)
                        }
                    }
                }
            }
        }
        Err(_)=>
        {
            Err(false)
        }
    }
}