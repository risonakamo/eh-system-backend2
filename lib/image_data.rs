// library for retrieving image data from file system structure

use std::path::{Path,PathBuf};
use std::fs::{read_dir,DirEntry};
use std::io;

use crate::types::image_data_types::AlbumList;

/// get albums of a path represented by a basename and a path.
pub fn getAlbumsWithBase(basepath:&str,path:&str)->AlbumList
{
    return getAlbums(Path::new(basepath).join(path));
}

/// get paths of ALL albums under the target path, relative to basepath. INCLUDES the initial base
/// path given. all paths will be in relative forme.
pub fn getSubAlbumsRec(basepath:&str,path:&str)->Vec<PathBuf>
{
    let mut subalbumPaths:Vec<PathBuf>=getSubAlbums(basepath,path).into_iter()
    .map(|x:PathBuf|->Vec<PathBuf> {
        return getSubAlbumsRec(basepath,x.to_str().unwrap());
    }).flatten().collect();

    subalbumPaths.push(Path::new(path).to_path_buf());

    return subalbumPaths;
}

/// get paths of all albums at the specified path relative to base dir, SINGLE LEVEL. returned paths will
/// also be relative to base dir.
pub fn getSubAlbums(basepath:&str,path:&str)->Vec<PathBuf>
{
    let targetpath:PathBuf=Path::new(basepath).join(path);

    return read_dir(targetpath).unwrap().filter_map(|x:io::Result<DirEntry>|->Option<PathBuf> {
        let thepath:PathBuf=x.unwrap().path();

        if !thepath.is_dir()
        {
            return None;
        }

        return Some(PathBuf::from(thepath.strip_prefix(basepath).unwrap()));
    }).collect();
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
    use super::getAlbumsWithBase;

    pub fn test()
    {
        let r=getAlbumsWithBase(r"testfiles2","");
        println!("{:#?}",r);
    }
}