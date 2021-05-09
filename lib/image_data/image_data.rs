// library for retrieving image data from file system structure

use std::path::{Path,PathBuf};
use std::fs::{read_dir,DirEntry};
use std::io;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::types::image_data_types::{AlbumList,FlatRawAlbum};

/// get flattened album list at specified base path. shuffles if shuffle is set.
pub fn getAlbumsWithBaseFlat(basepath:&str,path:&str,shuffle:bool)->FlatRawAlbum
{
    let mut albumlist=getAlbumsWithBase(basepath,path);

    if shuffle
    {
        albumlist.shuffle(&mut thread_rng());
    }

    return albumlist.into_iter().flatten().collect();
}

/// get albums of a path represented by a basename and a path.
pub fn getAlbumsWithBase(basepath:&str,path:&str)->AlbumList
{
    return getAlbums(Path::new(basepath).join(path));
}

/// get album image data of a full target path.
fn getAlbums(targetpath:PathBuf)->AlbumList
{
    let mut ownImages:Vec<String>=vec![];

    let mut subalbums:AlbumList=read_dir(targetpath).unwrap()
    // retrieve all sub directory paths
    .filter_map(|x:io::Result<DirEntry>|->Option<PathBuf> {
        let thepath:PathBuf=x.unwrap().path();

        if !thepath.is_dir()
        {
            ownImages.push(thepath.to_str().unwrap().to_string());
            return None;
        }

        return Some(thepath);
    })
    // recursive call getAlbums on subdirs
    .map(|x:PathBuf|->AlbumList {
        return getAlbums(x);
    })
    .flatten().collect();

    if ownImages.len()>0
    {
        subalbums.push(ownImages);
    }

    return subalbums;
}

pub mod test
{
    use super::{getAlbumsWithBase,getAlbumsWithBaseFlat};

    pub fn test()
    {
        let r=getAlbumsWithBase(r"testfiles2","");
        println!("{:#?}",r);
    }

    pub fn test2()
    {
        let r=getAlbumsWithBaseFlat("testfiles2","",true);
        println!("{:#?}",r);
    }
}