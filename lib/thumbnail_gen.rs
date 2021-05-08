/// thumbnail generation library

use std::path::{Path,PathBuf};
use std::fs::{read_dir,DirEntry};
use std::io;
use tokio::process::Command;

/// generate thumbnails for a single directory, relative to a base path. reconstructs the path structure
/// of basepath+imagepath in the thumbnail base path, and places images in that location.
pub fn genAlbumThumbnails(basepath:&str,thumbnailBasePath:&str,imagepath:&str)
{
    // full path to the target image dir
    let fullImagePath:PathBuf=Path::new(basepath).join(imagepath);

    // full path to the thumbnail output dir
    let thumbnailOutputDir:PathBuf=Path::new(thumbnailBasePath).join(imagepath);

    let imagePaths:Vec<PathBuf>=read_dir(fullImagePath).unwrap()
    .filter_map(|x:io::Result<DirEntry>|->Option<PathBuf> {
        let thepath:PathBuf=x.unwrap().path();

        if thepath.is_dir()
        {
            return None;
        }

        return Some(thepath);
    }).collect();

    println!("{:#?}",imagePaths);
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

pub mod test
{
    use super::genAlbumThumbnails;

    pub fn test()
    {
        genAlbumThumbnails(
            "testfiles2",
            "testthumbnaildata",
            "ctrlz77/double/1"
        );
    }
}