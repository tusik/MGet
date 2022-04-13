use mget::mget::DownloadOptions;
use clap::Parser;
#[derive(Parser, Debug)]
///A Mutil thread download tool 
///https://github.com/tusik/MGet
#[clap(author, version, about, long_about = None)]
pub struct Args{
    /// Download url
    #[clap(short, long)]
    url: String,
    /// Local storage file name, keep empty to use uri as file name
    #[clap(short, long)]
    file_name: Option<String>,
    /// Data unit: byte
    #[clap(short, long, default_value_t=1024*1024)]
    batch_size: u64,
    /// Overwrite local file
    #[clap(short, long)]
    overwrite: bool

}

pub async fn do_download(args:Args){
    let u:String = args.url;
    let d = DownloadOptions::new()
        .batch_size(args.batch_size)
        .url(u)
        .build().await;
    match d{
        Some(mut downloader) => {
            downloader.download().await;
        },
        None => {

        },
    }
}
#[tokio::main]
async fn main() {
    let args = Args::parse();
    do_download(args).await;
}
#[tokio::test]
async fn main_test(){
    let args = Args{ 
        url: "https://mirrors.aliyun.com/debian-cd/current/amd64/log/20220326/B_amd64_dvd.log".to_string(), 
        file_name: None, 
        batch_size: 1024*100,
        overwrite: false
     };
    do_download(args).await;
}
#[tokio::test]
async fn large_test(){
    let args = Args{ 
        url: "https://mirrors.aliyun.com/debian-cd/current/amd64/iso-cd/debian-11.3.0-amd64-netinst.iso".to_string(), 
        file_name: None, 
        batch_size: 50*1024*1024,
        overwrite: true
     };
    do_download(args).await;
}