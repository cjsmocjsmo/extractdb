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
    let mut stmt_jpg = conn.prepare("SELECT DISTINCT imghash FROM jpgs;").unwrap();
    let mut rows = stmt_jpg.query([]).unwrap();
    let mut img_hash_vec = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        
        let imghash1: String = row.get(2).unwrap();
        
        img_hash_vec.push(imghash1);
    };

    let mut stmt_png = conn.prepare("SELECT DISTINCT imghash FROM pngs;").unwrap();
    let mut rows = stmt_png.query([]).unwrap();
    while let Some(row) = rows.next().unwrap() {
        let imghash3: String = row.get(2).unwrap();
        img_hash_vec.push(imghash3);
    };

    let mut stmt_bmp = conn.prepare("SELECT DISTINCT imghash FROM bmps;").unwrap();
    let mut rows = stmt_bmp.query([]).unwrap();
    while let Some(row) = rows.next().unwrap() {
        let imghash2: String = row.get(2).unwrap();
        img_hash_vec.push(imghash2);
    };

    let jpg_out = env::var("DEDUPS_JPG").expect("DEDUPS_JPG not set");
    fs::create_dir_all(jpg_out.clone()).expect("Unable to create JPG directory");
    let png_out = env::var("DEDUPS_PNG").expect("DEDUPS_PNG not set");
    fs::create_dir_all(png_out.clone()).expect("Unable to create PNG directory");
    let bmp_out = env::var("DEDUPS_BMP").expect("DEDUPS_BMP not set");
    fs::create_dir_all(bmp_out.clone()).expect("Unable to create BMP directory");

    let mut new_meta_vec = Vec::new();
    for ihash in img_hash_vec {
        // let old_path = e.path().to_string_lossy().to_string();
        println!("old_path: {}", ihash);
        let mut stmt = conn.prepare("SELECT * FROM jpgs WHERE imghash = ? LIMIT 1").unwrap();
        let mut rows = stmt.query(&[&ihash]).unwrap();
        
        while let Some(row) = rows.next().unwrap() {
            let imgidz = row.get(1).unwrap();
            let imghashz = row.get(2).unwrap();
            let imgpathz = row.get(3).unwrap();
            let meta = types::Meta{
                imgid: imgidz,
                imghash: imghashz,
                imgpath: imgpathz,
            };
            new_meta_vec.push(meta);
        };

        let mut stmt2 = conn.prepare("SELECT * FROM pngs WHERE imghash = ? LIMIT 1").unwrap();
        let mut rows = stmt2.query(&[&ihash]).unwrap();
        while let Some(row) = rows.next().unwrap() {
            let imgidz = row.get(1).unwrap();
            let imghashz = row.get(2).unwrap();
            let imgpathz = row.get(3).unwrap();
            let meta2 = types::Meta{
                imgid: imgidz,
                imghash: imghashz,
                imgpath: imgpathz,
            };
            new_meta_vec.push(meta2);
        };

        let mut stm3 = conn.prepare("SELECT * FROM bmps WHERE imghash = ? LIMIT 1").unwrap();
        let mut rows = stm3.query(&[&ihash]).unwrap();
        while let Some(row) = rows.next().unwrap() {
            let imgidz = row.get(1).unwrap();
            let imghashz = row.get(2).unwrap();
            let imgpathz = row.get(3).unwrap();
            let meta3 = types::Meta{
                imgid: imgidz,
                imghash: imghashz,
                imgpath: imgpathz,
            };
            new_meta_vec.push(meta3);
        };
    };

    for meta in new_meta_vec {
        let old_path = meta.imgpath;
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
