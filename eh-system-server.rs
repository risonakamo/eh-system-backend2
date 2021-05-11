#![feature(proc_macro_hygiene,decl_macro)]
#![allow(non_snake_case)]

use rocket::{post,routes};
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

use eh_system::album_info::{getAlbumInfo};
use eh_system::types::album_info_types::AlbumInfo;
use eh_system::url_conversion::rebaseAlbumInfos;

static IMAGE_DATA_PATH:&str=r"C:\Users\ktkm\Desktop\h\cg";
static THUMBNAIL_API_PATH:&str="/thumbnaildata";

#[post("/get-album",format="text/plain",data="<albumpath>")]
fn getAlbum_api(albumpath:String)
{
    println!("{}",albumpath);
}

#[post("/get-album-info",format="text/plain",data="<albumpath>")]
fn getAlbumInfo_api(albumpath:String)->JsonValue
{
    let albuminfo:Vec<AlbumInfo>=getAlbumInfo(
        IMAGE_DATA_PATH,
        &albumpath
    );

    let albuminfoFixedUrls:Vec<AlbumInfo>=rebaseAlbumInfos(
        IMAGE_DATA_PATH,
        THUMBNAIL_API_PATH,
        albuminfo
    );

    return json!(albuminfoFixedUrls);
}

fn main()
{
    rocket::ignite()
    .mount("/",routes![getAlbum_api,getAlbumInfo_api])
        .launch();
}