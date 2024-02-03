use std::env;

pub fn set_env_vars() {
    let db_path = env::var("DUPS_DB");
    if db_path.is_err() {
        env::set_var("DUPS_DB", "/home/pipi/insertdb/dups.db");
    };
    let dedups_jpg = env::var("DEDUPS_JPG");
    if dedups_jpg.is_err() {
        env::set_var("DEDUPS_JPG", "/home/pipi/USB01/JPG");
    };
    let dedups_png = env::var("DEDUPS_PNG");
    if dedups_png.is_err() {
        env::set_var("DEDUPS_PNG", "/home/pipi/USB01/PNG");
    };
    let dedups_bmp = env::var("DEDUPS_BMP");
    if dedups_bmp.is_err() {
        env::set_var("DEDUPS_BMP", "/home/pipi/USB01/BMP");
    };
    let unable_to_open = env::var("UNABLE_TO_OPEN");
    if unable_to_open.is_err() {
        env::set_var("UNABLE_TO_OPEN", "/home/pipi/insertdb/unable_to_open/");
    };
}
