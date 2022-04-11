pub struct DownloadOptions{
    batch_size: u64,
    over_write: bool
}
impl DownloadOptions {
    pub fn new()->DownloadOptions{
        DownloadOptions{ 
            batch_size: 1024*1024, 
            over_write: false
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
}