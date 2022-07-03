use mget::mget::DownloadOptions;
use clap::Parser;
#[derive(Parser, Debug)]
///A Mutil thread download tool 
///https://github.com/tusik/MGet
#[clap(author, version, about, long_about = None)]
pub struct Args{
    /// Download url
    #[clap(parse(try_from_str=check_url))]
    url: String,
    /// Local storage file name, keep empty to use uri as file name
    #[clap(short, long)]
    file_name: Option<String>,
    /// Data unit: b,k,m,g
    #[clap(parse(try_from_str=check_size), short, long)]
    batch_size: u64,
    /// Overwrite local file
    #[clap(short, long)]
    overwrite: bool

}

fn check_url(s: &str) -> Result<String,String>{

    if s.to_lowercase().find("http://") == Some(0) || s.to_lowercase().find("https://") == Some(0) {
        Ok(s.to_string())
    }else{
        Err("No validate download url provide.".to_string())
    }
}

fn check_size(s: &str) -> Result<u64,String>{
    if s.len() >= 2{
        let unit = s.chars().last().unwrap();
        let size = s[..s.len()-1].parse::<u64>();
        match size{
            Ok(size) => {
                match unit{
                    'b' => Ok(size),
                    'k' => Ok(size*1024),
                    'm' => Ok(size*1024*1024),
                    'g' => Ok(size*1024*1024*1024),
                    _ => Err("No validate data unit provide.".to_string())
                }
            },
            Err(_) => Err("No validate data unit provide.".to_string())
        }
            
    }else{
        Err("No validate data unit provide.".to_string())
    }
}

pub async fn do_download(args:Args){

    let u:String = args.url;
    let d = DownloadOptions::new()
        .batch_size(args.batch_size)
        .url(u)
        .overwrite(args.overwrite)
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
        url: "https://mirrors.aliyun.com/ubuntu/pool/main/b/busybox/busybox_1.30.1-6ubuntu3.1.dsc".to_string(), 
        file_name: None, 
        batch_size: 10,
        overwrite: true
     };
    do_download(args).await;
}
#[tokio::test]
async fn large_test(){
    let args = Args{ 
        url: "https://mirrors.aliyun.com/linux-kernel/v5.x/patch-5.9.xz".to_string(), 
        file_name: None, 
        batch_size: 5*1024*1024,
        overwrite: true
     };
    do_download(args).await;
}