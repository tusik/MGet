use MGet::mget::downloader::{self, Downloader};
#[tokio::main]
async fn main() {
    println!("Hello, MGet!");
    
    let u:String = "https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-5.17.1.tar.xz".parse().unwrap();
    let size = Downloader::get_range(u.clone()).await;
    let down = Downloader::new(u.clone());

    print!("{}",size.unwrap());
    // Downloader::range_check(u);
}
