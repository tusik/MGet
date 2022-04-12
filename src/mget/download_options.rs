use super::Downloader;
#[derive(Debug, Clone)]
pub struct DownloadOptions{
    pub(crate) batch_size: u64,
    pub(crate) over_write: bool,
    pub(crate) download_url: String,
    pub(crate) write_file_name: String
}
impl DownloadOptions {
    pub fn new()->DownloadOptions{
        DownloadOptions{ 
            batch_size: 1024*1024, 
            over_write: false,
            download_url: String::new(),
            write_file_name: String::new()
        }
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
    pub async fn build(&self) -> Option<Downloader>{
        let mut d = Downloader::new();
        d.options(self.clone());
        if self.write_file_name.is_empty(){
            let file_path = self.download_url.clone(); 
            let file_path:Vec<&str> = file_path.split("/").collect();
            let file_os_file_name: String = file_path.last().unwrap().to_string();
            d.file = Downloader::open_file(file_os_file_name,self.over_write).await.map_or(None, |v|Some(v));

        }else{
            d.file = Downloader::open_file(self.write_file_name.clone(),self.over_write).await.map_or(None, |v|Some(v));
        }
        Some(d)
    }
}