use std::{sync::Arc, io::{SeekFrom, Read, Error, self}};
use bytes::Bytes;
use reqwest::{Method, Url};
use tokio::{fs::{File, OpenOptions}, io::{AsyncSeekExt, AsyncWriteExt}, sync::Mutex};
use futures::{self, stream::FuturesUnordered, StreamExt};
pub struct Downloader{
    file_name:String,
    file: Option<Arc<Mutex<File>>>,
    url: String,
    batch_size : u64,
}

fn read_line() -> String{
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to parse int. Please only enter digits.");
    s
}

impl Downloader {
    
    pub async fn file(&mut self, file_path:String){
        self.file_name = file_path.clone();
        self.file = Downloader::open_file(file_path.clone(),false).await.map_or(None, |v|Some(v))
    }
    pub fn batch_size(&mut self, batch_size:u64){
        self.batch_size = batch_size;
    }
    pub async fn new(url:String) -> Option<Downloader>{
        let u = Url::parse(&url);
        match u {
            Ok(url_item) => {
                let file_path = url_item.path().to_string(); 
                let file_path:Vec<&str> = file_path.split("/").collect();
                let file_os_file_name: String = file_path.last().unwrap().to_string();
                Some(
                    Downloader{
                        file_name: file_os_file_name.clone(),
                        file: Downloader::open_file(file_os_file_name.clone(),false).await.map_or(None, |v|Some(v)),
                        batch_size: 1024000,
                        url
                    }
                )
            },
            Err(_) => None,
        }
        
    }
    pub async fn open_file(path:String,overwrite:bool) ->Result<Arc<Mutex<File>>,Error> {
        let mut op = OpenOptions::new();
        let _file = op.read(true)
            .write(true)
            .create_new(!overwrite)
            .open(path.clone())
            .await;
        match _file {
            Ok(f) => {
                let file = Arc::new(Mutex::new(f));
                return Ok(file);
            },
            Err(e) => {
                println!("{}",e);
                Err(e)
            },
        }
    }
    pub async fn get_range(url:String) -> Option<u64>{
        let client = reqwest::Client::new();
        
        let rep = client.request(Method::HEAD, url).header("user-agent", "Mget-rs").send().await;
        match rep {
            Ok(response) => {
                let head = &response.headers();
                let a_r = head.get("Accept-Ranges");
                let mut head_size = head.get("content-length");
                if head_size.is_none(){
                    head_size = head.get("Content-Length");
                }
                let size = head_size.unwrap().to_str()
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
        let resp = client.get(url)
            .header("Range", format!("bytes={}-{}",start,end))
            .header("user-agent", "Mget-rs")
            .send().await;
        
        match resp {
            Ok(response) => {
                let data = response.bytes().await.ok().unwrap();
                println!("{}-{}",start,end);
                Downloader::write_to_file(file, start, &data).await;
                Some(data)
            },
            Err(e) => {
                print!("{}",e);
                return None
            },
        }
    }
    pub async fn write_to_file(file:Arc<Mutex<File>>,start:u64,data:&Bytes){
        let mut file_p = file.lock().await;
        file_p.seek(SeekFrom::Start(start as u64)).await.unwrap();
        let byte_data: Result<Vec<_>, _> = data.bytes().collect();
        let byte_data = byte_data.expect("Unable to read data");
        file_p.write_all(&byte_data).await.expect("Unable to write data");
    }
    pub async fn download(&mut self){
        if self.file.is_none(){
            return;
        }
        let range_check = Downloader::get_range(self.url.to_string()).await;
        let mut futs = FuturesUnordered::new();
        let mut index = 0;
        if range_check.is_some() {
            let max_size = range_check.unwrap();
            while index * self.batch_size < max_size {
                let f = self.file.clone();
                let u = self.url.clone();
                let download_batch_size = self.batch_size.clone();
                let task = tokio::spawn(async move {
                        Downloader::download_byte(
                            u,
                            index*download_batch_size, 
                            std::cmp::min((index+1) * download_batch_size,max_size),
                            f.unwrap()
                        ).await;
                });
                futs.push(task); 
                if futs.len() == 3 {
                    futs.next().await;
                }
                index+=1;
            }
            while let Some(_) = futs.next().await {
                // outputs.push(item);
            }
                        
        }else{
            
        }

        
    }
   
}
