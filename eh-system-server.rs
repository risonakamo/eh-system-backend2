#![feature(proc_macro_hygiene,decl_macro)]
#![allow(non_snake_case)]

use rocket::{post,routes};
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

use eh_system::album_info::{getAlbumInfo};
use eh_system::types::album_info_types::AlbumInfo;
use eh_system::url_conversion::rebaseAlbumInfos;

#[post("/get-album",format="text/plain",data="<albumpath>")]
fn getAlbum_api(albumpath:String)
{
    println!("{}",albumpath);
}

#[post("/get-album-info",format="text/plain",data="<albumpath>")]
fn getAlbumInfo_api(albumpath:String)->JsonValue
{
    let albuminfo:Vec<AlbumInfo>=getAlbumInfo(
        r"testfiles2",
        &albumpath
    );

    let albuminfoFixedUrls:Vec<AlbumInfo>=rebaseAlbumInfos(
        r"testfiles2",
        "/thumbnaildata",
        albuminfo
    );

    println!("{:#?}",albuminfoFixedUrls);

    return json!(albuminfoFixedUrls);
}

fn main()
{
    rocket::ignite()
        .mount("/",routes![getAlbum_api,getAlbumInfo_api])
        .launch();
}