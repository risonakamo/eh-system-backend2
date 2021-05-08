/// thumbnail generation library

use std::path::{Path,PathBuf};
use std::fs::{read_dir,DirEntry,create_dir_all};
use std::io;
use tokio::process::{Command};
use tokio::task::JoinHandle;
use tokio::join;

/// generate thumbnails for a single directory, relative to a base path. reconstructs the path structure
/// of basepath+imagepath in the thumbnail base path, and places images in that location.
pub async fn genAlbumThumbnails(basepath:&str,thumbnailBasePath:&str,imagepath:&str)
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

    if imagePaths.len()==0
    {
        return;
    }

    create_dir_all(&thumbnailOutputDir).unwrap();

    // TODO: generate thumbnails in batches so dont break system with many spawned processes
    genMultipleThumbnails(
        imagePaths,
        thumbnailOutputDir.to_str().unwrap().to_string(),
        4,
        200
    ).await;
}

/// generate thumbnails for a vector of targets. places all into the same output directory. specify batch
/// size for number of simultaneous ffmpeg processes.
async fn genMultipleThumbnails(targets:Vec<PathBuf>,outputdir:String,batchSize:u32,height:u32)
{
    let thumbnailTasks:Vec<JoinHandle<()>>=targets.into_iter()
    .map(|x:PathBuf|->JoinHandle<()> {
        let target:String=x.clone().to_str().unwrap().to_string();
        let outputdir2:String=outputdir.clone();
        let height2:u32=height.clone();

        println!("generating {}",target);

        return tokio::spawn(async move {
            genThumbnail(
                target,
                outputdir2,
                height2
            ).await;
        });
    }).collect();

    for x in thumbnailTasks
    {
        join!(x);
    }
}

/// generate a thumbnail for the given target image and place it into the target output folder,
/// with the same name as the original, but with png file extension.
async fn genThumbnail(target:String,outputDir:String,height:u32)
{
    let outputPath:String=Path::new(&outputDir).join(
        Path::new(&target).file_stem().unwrap()
    ).with_extension("png").to_str().unwrap().to_string();

    let mut ffmpeg=Command::new("ffmpeg")
        .arg("-y")
        .args(&["-loglevel","error"])
        .args(&["-i",&target])
        .args(&["-vf",&format!("scale={}:-1",height)])
        .args(&["-frames:v","1"])
        .arg(outputPath)
        .spawn()
        .unwrap();

    ffmpeg.wait().await.unwrap();
}

pub mod test
{
    use super::genAlbumThumbnails;

    pub async fn test()
    {
        genAlbumThumbnails(
            "testfiles2",
            "testthumbnaildata",
            "ctrlz77/double/1"
        ).await;
    }
}