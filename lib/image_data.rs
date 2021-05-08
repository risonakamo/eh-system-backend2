/// library for retrieving image data from file system structure

use std::path::{Path,PathBuf};
use std::fs::{read_dir,DirEntry};
use std::io;

use crate::types::image_data_types::AlbumList;

pub fn getAlbums(basepath:&str,path:&str)->AlbumList
{
    let targetpath:PathBuf=Path::new(basepath).join(Path::new(path));

    let subdirs:Vec<PathBuf>=read_dir(targetpath).unwrap()
    .filter_map(|x:io::Result<DirEntry>|->Option<PathBuf> {
        let thepath:PathBuf=x.unwrap().path();

        if !thepath.is_dir()
        {
            return None;
        }

        return Some(thepath);
    }).collect();

    println!("{:#?}",subdirs);

    return vec![vec![]];
}

pub mod test
{
    use super::getAlbums;

    pub fn test()
    {
        getAlbums("testfiles2","ctrlz77/double/1");
    }
}