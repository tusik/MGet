use std::{sync::{Arc}, io::SeekFrom};
use bytes::Bytes;
use tokio::{fs::{File, OpenOptions}, io::{AsyncSeekExt, AsyncWriteExt}, sync::Mutex};

pub struct Downloader{
    file_name:String,
    file : Arc<Mutex<File>>,
    batch_size : u64
}
impl Downloader {
    pub async fn new(path:String) -> Downloader{
        Downloader{
            file_name:path.clone(),
            file: Downloader::file(path).await,
            batch_size: 10240
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
    pub async fn download_byte(url:String,start:u64,end:u64,file:Arc<Mutex<File>>)->Option<Bytes>{
        let client = reqwest::Client::new();
        let resp = client.get(url).header("Range", format!("bytes={}-{}",start,end)).send().await;
        
        match resp {
            Ok(response) => {
                let data = response.bytes().await.ok().unwrap();
                Downloader::write_to_file(file, start, &data).await;
                Some(data)

            },
            Err(e) => {
                print!("{}",e);
                return None
            },
        }
    }
    pub async fn write_to_file(file:Arc<Mutex<File>>,start:u64,data:&[u8]){
        let mut file_p = file.lock().await;
        file_p.seek(SeekFrom::Start(start as u64)).await.unwrap();
        file_p.write_all(data).await;
    }
    pub async fn download(&self,url:String){
        let mut handles = vec![];
        let range_check = Downloader::get_range(url.to_string()).await;
    
        let mut index = 0;
        let batch_size = 1024000;
        if range_check.is_some() {
            let max_size = range_check.unwrap();
            while index*batch_size<max_size {
                let f = self.file.clone();
                let u = url.clone();
                let download_batch_size = self.batch_size.clone();
                handles.push(tokio::spawn(async move{
                    let data = 
                    Downloader::download_byte(
                        u,
                        index*download_batch_size, 
                        std::cmp::min((index+1)*download_batch_size,max_size),
                        f
                    ).await.unwrap();
                    // Downloader::write_to_file(f,index*download_batch_size,data.as_ref()).await;
                })); 
                
                // inner_file.write_all(data.as_ref()).await;
                index+=1;
            }
            
        }else{
            
        }
        futures::future::join_all(handles).await;
        
    }
   
}
