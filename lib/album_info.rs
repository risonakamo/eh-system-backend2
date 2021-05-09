// lib for retrieving album information

use std::path::{PathBuf,Path};
use std::fs::{DirEntry,read_dir,metadata,Metadata};
use chrono::{DateTime,Local};
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::image_data::image_data::{getAlbumsWithBaseFlat};

use crate::types::album_info_types::{AlbumInfo,ImmediateAlbumInfo};
use crate::types::image_data_types::{FlatRawAlbum};

/// get album info array for all albums at the target path relative to base path.
fn getAlbumInfo(basepath:&str,path:&str)->Vec<AlbumInfo>
{

}

/// get album info for target basepath/path
fn getAlbumInfoSingle(basepath:&str,target:&str)->AlbumInfo
{
    let fulltarget:PathBuf=Path::new(basepath).join(target);

    let albumImages:FlatRawAlbum=getAlbumsWithBaseFlat(basepath,target,false);

    let imInfo:ImmediateAlbumInfo=getImmediateAlbumInfo(&fulltarget);

    return AlbumInfo {
        title:fulltarget.file_name().unwrap().to_str().unwrap().to_string(),
        items:albumImages.len() as u32,
        immediateItems:imInfo.immediateItems,
        img:albumImages.choose(&mut thread_rng()).unwrap().clone(),
        date:imInfo.date,
        album:imInfo.album
    };
}

/// retrieve immediate album info for a target album (full path)
fn getImmediateAlbumInfo(target:&PathBuf)->ImmediateAlbumInfo
{
    let mut totalCount:u32=0;
    let mut isAlbum:bool=true;

    for x in read_dir(target).unwrap()
    {
        let thepath:PathBuf=x.unwrap().path();

        // if there is a dir in the album, it is not a true album (true albums have
        // only images)
        if thepath.is_dir()
        {
            isAlbum=false;
        }

        totalCount+=1;
    }

    let mdata:Metadata=metadata(target).unwrap();
    let date:DateTime<Local>=DateTime::from(mdata.modified().unwrap());
    let datestring:String=date.format("%Y-%m-%d %T").to_string();

    return ImmediateAlbumInfo {
        immediateItems:totalCount,
        album:isAlbum,
        date:datestring
    };
}