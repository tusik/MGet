use std::{sync::Arc, io::{SeekFrom, Error}};
use bytes::Bytes;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Method;
use tokio::{fs::{File, OpenOptions}, io::{AsyncSeekExt, AsyncWriteExt}, sync::Mutex};
use futures::{self, stream::FuturesUnordered, StreamExt};

use super::DownloadOptions;
pub struct Downloader{
    pub file: Option<Arc<Mutex<File>>>,
    options: Option<DownloadOptions>,
    progress_bar: Option<Arc<Mutex<ProgressBar>>>
}   

impl Downloader {

    pub fn options(&mut self,op:DownloadOptions){
        self.options = Some(op);
    }

    pub fn new() -> Downloader{

        Downloader{
            file: None,
            options: None,
            progress_bar: None
        }
    }

    pub async fn open_file(path: String, file_size: usize, overwrite: bool) ->Result<Arc<Mutex<File>>,Error> {
        let mut op = OpenOptions::new();
        let _file ;
        if overwrite {
            _file = op.read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open(path.clone())
            .await;
        }else{
            _file = op.read(true)
            .write(true)
            .create_new(true)
            .open(path.clone())
            .await;
        }
        match _file {
            Ok(f) => {
                let re_size = f.set_len(file_size as u64).await;
                match re_size {
                    Ok(_) => {
                        let file = Arc::new(Mutex::new(f));
                        return Ok(file);
                    },
                    Err(e) => {
                        println!("{}",e);
                        Err(e)
                    }
                }
                
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

    pub async fn download_byte(url:String,start:u64,end:u64,file:Arc<Mutex<File>>)->Option<u64>{
        let client = reqwest::Client::new();
        let resp = client.get(url)
            .header("Range", format!("bytes={}-{}",start,end))
            .header("user-agent", "Mget-rs")
            .send().await;
        
        match resp {
            Ok(response) => {
                // TODO: error handler
                let data = &response.bytes().await.unwrap();

                Downloader::write_to_file(file, start, data).await;
                Some(1)
            },
            Err(e) => {
                print!("{}",e);
                return None
            },
        }
    }

    pub async fn write_to_file(file:Arc<Mutex<File>>,start:u64,data:&Bytes){
        let mut file_p = file.lock().await;
        let _seek_pos = file_p.seek(SeekFrom::Start(start)).await.unwrap();

        file_p.write_all(data).await.expect("Unable to write data");

        // file_p.write_all(&byte_data).await.expect("Unable to write data");
    }

    pub async fn download(&mut self){
        if self.options.is_none()
            ||self.file.is_none(){
            return;
        }
        let op: DownloadOptions = self.options.clone().unwrap();
        let range_check = Downloader::get_range(op.download_url.clone()).await;
        let mut futs = FuturesUnordered::new();
        let mut index = 0;
        let sty = ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .progress_chars("#>-");
            
        let pb = ProgressBar::new(range_check.unwrap());
        pb.set_style(sty.clone());
        self.progress_bar = Some(Arc::new(Mutex::new(pb)));

        if range_check.is_some() {
            let max_size = range_check.unwrap();
            while index * op.batch_size < max_size {
                let f = self.file.as_ref().unwrap().clone();
                let u: String = op.download_url.clone();
                let download_batch_size = op.batch_size.clone();
                let _pb = self.progress_bar.as_ref().unwrap().clone();
                let task = tokio::spawn(async move {
                    let mut start = index*download_batch_size;
                    if index > 0 {
                        start += 1;
                    }
                    Downloader::download_byte(
                        u,
                        start, 
                        std::cmp::min((index+1) * download_batch_size,max_size),
                        f
                    ).await;
                    let _pb = _pb.lock().await;
                    _pb.set_position(_pb.position()+download_batch_size);
                });
                futs.push(task); 
                if futs.len() == self.options.as_ref().unwrap().download_threads {
                    futs.next().await;
                }
                index+=1;
            }

            while let Some(_) = futs.next().await {}
            self.progress_bar.as_ref().unwrap().lock().await.finish();
                        
        }else{
            
        }
        
    }
   
}
