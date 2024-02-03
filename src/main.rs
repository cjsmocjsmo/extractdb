use std::env;
// extern crate img_hash;
use std::fs;

pub mod envvars;
pub mod types;

fn main() {
    envvars::set_env_vars();
    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     println!("Usage: {} <apath>", args[0]);
    //     return;
    // }
    // let apath = &args[1];
    let db_path = env::var("DUPS_DB").expect("DUPS_DB not set");
    let conn = rusqlite::Connection::open(db_path).expect("Unable to open database");
    let mut stmt = conn.prepare("SELECT * FROM jpgs;").unwrap();
    let mut rows = stmt.query([]).unwrap();
    let mut old_img_path_vec = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let imgid: String = row.get(1).unwrap();
        let imghash: String = row.get(2).unwrap();
        let imgpath: String = row.get(3).unwrap();
        let meta = types::Meta {
            imgid: imgid,
            imghash: imghash,
            imgpath: imgpath,
        };
        old_img_path_vec.push(meta);
    }

    let jpg_out = env::var("DEDUPS_JPG").expect("DEDUPS_JPG not set");
    fs::create_dir_all(jpg_out.clone()).expect("Unable to create JPG directory");
    let png_out = env::var("DEDUPS_PNG").expect("DEDUPS_PNG not set");
    fs::create_dir_all(png_out.clone()).expect("Unable to create PNG directory");
    let bmp_out = env::var("DEDUPS_BMP").expect("DEDUPS_BMP not set");
    fs::create_dir_all(bmp_out.clone()).expect("Unable to create BMP directory");

    for e in old_img_path_vec {
        // let old_path = e.path().to_string_lossy().to_string();
        let old_path = e.imgpath;
        let fname = split_fname(old_path.clone());
        let ext = split_ext(old_path.clone());

        if ext == "jpg".to_string() {
            let new_path = format!("{}/{}", jpg_out.clone(), fname);
            // open old_path and read it into a buffer
            let buffer = fs::read(old_path.clone()).expect("Unable to read file");
            // create new_path and write buffer to it
            fs::write(new_path, buffer).expect("Unable to write file");
        } else if ext == "png".to_string() {
            let new_path = format!("{}/{}", png_out.clone(), fname);
            // open old_path and read it into a buffer
            let buffer = fs::read(old_path.clone()).expect("Unable to read file");
            // create new_path and write buffer to it
            fs::write(new_path, buffer).expect("Unable to write file");
        } else if ext == "bmp".to_string() {
            let new_path = format!("{}/{}", bmp_out.clone(), fname);
            // open old_path and read it into a buffer
            let buffer = fs::read(old_path.clone()).expect("Unable to read file");
            // create new_path and write buffer to it
            fs::write(new_path, buffer).expect("Unable to write file");
        }
    }
}

pub fn split_fname(apath: String) -> String {
    let fname_split = apath.split("/").collect::<Vec<&str>>();
    let fname = fname_split.last().unwrap().to_string();

    fname
}

pub fn split_ext(apath: String) -> String {
    let ext_split = apath.split(".").collect::<Vec<&str>>();
    let ext = ext_split.last().unwrap().to_string();

    ext
}
