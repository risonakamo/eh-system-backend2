/// thumbnail generation library

use std::path::{Path,PathBuf};
use std::fs::{read_dir,DirEntry,create_dir_all};
use std::io;
use tokio::process::{Command,Child};
use tokio::task::JoinHandle;
use tokio::join;
use itertools::{Itertools,IntoChunks};
use std::vec::IntoIter;

/// generate thumbnails for all albums under the target image path.
pub fn genAlbumThumbnailsRec(basepath:&str,thumbnailBasePath:&str,imagepath:&str,batchsize:u32,height:u32)
{

}

/// generate thumbnails for a single directory, relative to a base path. reconstructs the path structure
/// of basepath+imagepath in the thumbnail base path, and places images in that location.
async fn genAlbumThumbnails(basepath:&str,thumbnailBasePath:&str,
    imagepath:&str,batchsize:u32,height:u32)
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

    genMultipleThumbnailsBatches(
        imagePaths,
        thumbnailOutputDir.to_str().unwrap().to_string(),
        batchsize,
        height
    ).await;
}

/// generate thumbnails for vector of targets, with a max number of ffmpeg processes.
async fn genMultipleThumbnailsBatches(targets:Vec<PathBuf>,outputdir:String,
    batches:u32,height:u32)
{
    let batchCount:u32=(targets.len() as f32/batches as f32).ceil() as u32;
    let chunks:IntoChunks<IntoIter<PathBuf>>=targets.into_iter().chunks(batches as usize);

    for (i,x) in chunks.into_iter().enumerate()
    {
        println!("generating {}/{}",i+1,batchCount);

        genMultipleThumbnails(
            x.collect(),
            outputdir.clone(),
            height
        ).await;
    }
}

/// generate thumbnails for a vector of targets. places all into the same output directory.
async fn genMultipleThumbnails(targets:Vec<PathBuf>,outputdir:String,height:u32)
{
    let thumbnailTasks:Vec<JoinHandle<()>>=targets.into_iter()
    .map(|x:PathBuf|->JoinHandle<()> {
        let outputdir2:String=outputdir.clone();

        return tokio::spawn(async move {
            genThumbnail(
                x.to_str().unwrap(),
                &outputdir2,
                height
            ).await;
        });
    }).collect();

    for x in thumbnailTasks
    {
        let _res=join!(x);
    }
}

/// generate a thumbnail for the given target image and place it into the target output folder,
/// with the same name as the original, but with png file extension.
async fn genThumbnail(target:&str,outputDir:&str,height:u32)
{
    let outputPath:String=Path::new(&outputDir).join(
        Path::new(&target).file_stem().unwrap()
    ).with_extension("png").to_str().unwrap().to_string();

    let mut ffmpeg:Child=Command::new("ffmpeg")
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
    use crate::image_data::getSubAlbums;

    pub async fn test()
    {
        genAlbumThumbnails(
            "testfiles2",
            "testthumbnaildata",
            "ctrlz77/double/1",
            3,
            200
        ).await;
    }

    pub async fn test2()
    {
        let r=getSubAlbums(
            "testfiles2",
            "ctrlz77"
        );

        println!("{:#?}",r);
    }
}