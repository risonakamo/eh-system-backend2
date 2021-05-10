// lib for converting paths to api paths

use std::path::Path;
use path_slash::PathExt;

use crate::types::album_info_types::AlbumInfo;

/// rebase relevant paths in an array of AlbumInfos.
pub fn rebaseAlbumInfos(initialBase:&str,newbase:&str,infos:Vec<AlbumInfo>)->Vec<AlbumInfo>
{
    return infos.into_iter().map(|x:AlbumInfo|->AlbumInfo {
        return AlbumInfo {
            img:rebasePath(
                initialBase,
                newbase,
                &x.img
            ),
            ..x
        };
    }).collect();
}

/// convert path relative to some base to another base. assumes that imagepath has the base
/// specified by initialBase. otherwise, would just need to join newBase and imagePath.
/// also returns canonicalised forme.
fn rebasePath(initialBase:&str,newBase:&str,imagepath:&str)->String
{
    return Path::new(newBase).join(
        Path::new(imagepath).strip_prefix(initialBase).unwrap()
    ).to_slash().unwrap();
}