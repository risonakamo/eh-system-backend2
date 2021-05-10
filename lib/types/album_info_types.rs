use serde::{Serialize};

/// album info for a album from file
#[derive(Debug,Serialize)]
pub struct AlbumInfo
{
    pub title:String,
    pub items:u32,
    pub immediateItems:u32,
    pub img:String,
    pub date:String,
    pub album:bool
}

/// album info subset
pub struct ImmediateAlbumInfo
{
    pub immediateItems:u32,
    pub album:bool,
    pub date:String
}