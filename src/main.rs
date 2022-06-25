use mget::mget::DownloadOptions;
use clap::Parser;
use regex::Regex;
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
    /// Data unit: byte
    #[clap(parse(try_from_str=check_size), short, long)]
    batch_size: u64,
    /// Overwrite local file
    #[clap(short, long)]
    overwrite: bool

}

fn check_url(s: &str) -> Result<String,String>{
    let re = Regex::new("(^http://)|(^https://)").unwrap();
    if re.is_match(s) {
        Ok(s.to_string())
    }else{
        Err("No validate download url provide.".to_string())
    }
}

fn check_size(s: &str) -> Result<u64,String>{
    let batch_re = Regex::new("^[0-9]+").unwrap();
    let unit_re = Regex::new("[bkmgt|BKMGT]$").unwrap();
    let unit = unit_re.find_iter(s).map(
        |x| x.as_str()
    ).collect::<Vec<&str>>();
    let batch = batch_re.find_iter(s).map(|x| x.as_str().parse().unwrap()).collect::<Vec<u64>>();
    print!("{:?}",batch);
    if batch.len() == 1 && unit.len() == 1 {
        Ok(batch[0] * match unit[0] {
            "b"|"B" => 1,
            "k"|"K" => 1024,
            "m"|"M" => 1024 * 1024,
            "g"|"G" => 1024 * 1024 * 1024,
            "t"|"T" => 1024 * 1024 * 1024 * 1024,
            _ => 1
        })
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
    check_size("10m");
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
        url: "https://mirrors.aliyun.com/debian-cd/current/amd64/iso-cd/debian-11.3.0-amd64-netinst.iso".to_string(), 
        file_name: None, 
        batch_size: 5*1024*1024,
        overwrite: true
     };
    do_download(args).await;
}