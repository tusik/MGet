use super::Downloader;
extern crate num_cpus;
#[derive(Debug, Clone)]
pub struct DownloadOptions{
    pub(crate) batch_size: u64,
    pub(crate) over_write: bool,
    pub(crate) download_url: String,
    pub(crate) write_file_name: String,
    pub(crate) download_threads: usize,
}
impl DownloadOptions {
    pub fn new()->DownloadOptions{
        let d = DownloadOptions{ 
            batch_size: 1024*1024, 
            over_write: false,
            download_url: String::new(),
            write_file_name: String::new(),
            download_threads: num_cpus::get()/2
        };
        
        return d;
    }
    pub fn batch_size(&mut self,size:u64) -> &mut DownloadOptions{
        self.batch_size = size;
        self
    }
    pub fn overwrite(&mut self,ov:bool) -> &mut DownloadOptions{
        self.over_write = ov;
        self
    }
    pub fn url(&mut self,download_url:String) -> &mut DownloadOptions{
        self.download_url = download_url;
        self
    }
    pub fn file_name(&mut self,f_n:String) -> &mut DownloadOptions{
        self.write_file_name = f_n;
        self
    }
    pub fn threads(&mut self,t:usize) -> &mut DownloadOptions{
        self.download_threads = t;
        self
    }
    pub async fn build(&self) -> Option<Downloader>{
        let mut d = Downloader::new();
        d.options(self.clone());
        let filename ;
        if self.write_file_name.is_empty(){
            let file_path = self.download_url.clone(); 
            let file_path:Vec<&str> = file_path.split("/").collect();
            let file_os_file_name: String = file_path.last().unwrap().to_string();
            filename = file_os_file_name;

        }else{
            filename = self.write_file_name.clone();
        }
        let file_size = Downloader::get_range(self.download_url.clone()).await;
        println!("Start Download {}, {} task",&filename,self.download_threads);
        match file_size {
            Some(size) => {
                d.file = Downloader::open_file(filename,size as usize,self.over_write).await.map_or(None, |v|Some(v));
            },
            None => {
                panic!("can not get File!");
            },
        }
        
        Some(d)
    }
}