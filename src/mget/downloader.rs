use std::sync::{Arc, Mutex};
use tokio::fs::{File, OpenOptions};

pub struct Downloader{
    file_name:String,
    file : Arc<Mutex<File>>
}
impl Downloader {
    pub async fn new(path:String) -> Downloader{
        Downloader{
            file_name:path.clone(),
            file: Downloader::file(path).await
        }
    }
    pub async fn file(path:String) ->Arc<Mutex<File>> {
        let mut op = OpenOptions::new();
        let file = Arc::new(Mutex::new(op.read(true)
            .write(true)
            .create_new(true)
            .open(path.clone()).await.unwrap()));
        file
    }
    pub async fn get_range(url:String) -> Option<u64>{
        let client = reqwest::Client::new();
        let rep = client.head(url).send().await;
        match rep {
            Ok(response) => {
                let a_r = &response.headers().get("Accept-Ranges");
                let size = &response.headers().get("Content-Length").unwrap().to_str()
                        .map_or(0, |v|v.parse::<u64>().map_or(0, |s|s));
                match a_r {
                    Some(v) => {
                        if  v.to_str().unwrap().eq("bytes") {
                            return Some(size.clone())
                        }
                        return None
                    },
                    None => return None,
                }
                
            },
            Err(e) => {
                panic!("{}",e);
            },
        }
    }
   
}
