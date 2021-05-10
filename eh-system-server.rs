#![feature(proc_macro_hygiene,decl_macro)]
#![allow(non_snake_case)]

use rocket::{post,routes};

#[post("/get-album",format="text/plain",data="<albumpath>")]
fn getAlbum_api(albumpath:String)
{
    println!("{}",albumpath);
}

#[post("/get-album-info",format="text/plain",data="<albumpath>")]
fn getAlbumInfo_api(albumpath:String)
{
    println!("info {}",albumpath);
}

fn main()
{
    rocket::ignite()
        .mount("/",routes![getAlbum_api,getAlbumInfo_api])
        .launch();
}