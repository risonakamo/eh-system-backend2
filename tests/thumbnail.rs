#![allow(non_snake_case)]

use tokio::process::Command;
use std::path::Path;

#[tokio::main]
async fn main()
{
    tokio::join!(
        genThumbnail("testfiles/test1.png","output",200),
        genThumbnail("testfiles/test2.jpg","output",200),
        genThumbnail("testfiles/test3.jpg","output",200),
        genThumbnail("testfiles/test4.webm","output",200),
        genThumbnail("testfiles/test5.gif","output",200),
        genThumbnail("testfiles/test6.mp4","output",200),
        genThumbnail("testfiles/test7.mp4","output",200),
        genThumbnail("testfiles/test8.mp4","output",200),
    );
}

/// generate a thumbnail for the given target image and place it into the target output folder,
/// with the same name as the original, but with png file extension.
async fn genThumbnail(target:&str,outputDir:&str,height:u32)
{
    let outputPath:String=Path::new(outputDir).join(
        Path::new(target).file_stem().unwrap()
    ).with_extension("png").to_str().unwrap().to_string();

    let mut thumbnailjob=Command::new("ffmpeg")
        .arg("-y")
        .args(&["-loglevel","error"])
        .args(&["-i",target])
        .args(&["-vf",&format!("scale={}:-1",height)])
        .args(&["-frames:v","1"])
        .arg(outputPath)
        .spawn()
        .unwrap();

    thumbnailjob.wait().await.unwrap();
}