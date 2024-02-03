// use img_hash::ImageHash;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub imgid: String,
    pub imghash: String,
    pub imgpath: String,
}

// #[derive(Clone, Debug)]
// pub struct ImgHashStruct {
//     pub hash: ImageHash,
// }
