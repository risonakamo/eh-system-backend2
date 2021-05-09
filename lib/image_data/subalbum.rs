// sub album data functions

use std::path::{Path,PathBuf};
use std::fs::{read_dir,DirEntry};
use std::io;

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