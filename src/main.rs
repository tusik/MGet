use MGet::mget::downloader::{self, Downloader};
#[tokio::main]
async fn main() {
    println!("Hello, MGet!");
    
    let u:String = "https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-5.17.1.tar.xz".parse().unwrap();
    let size = Downloader::get_range(u.clone()).await;
    print!("{}",size.unwrap());
    let down = Downloader::new("linux-5.17.1.tar.xz".to_string()).await;
    down.download(u).await;
    
    // Downloader::range_check(u);
}
