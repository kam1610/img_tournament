use std::fs;
use std::path::Path;
use std::path::PathBuf;
use chrono::{DateTime, Datelike, Local};
// list_files //////////////////////////////////////////////
fn list_files(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)?{
        let entry = entry?;
        if entry.file_type()?.is_file(){
            files.push(entry.path());
        }
    }
    return Ok(files)
}
// filter_by_year_month ////////////////////////////////////
fn filter_by_year_month(paths: Vec<PathBuf>, y: i32, m: u32) -> Vec<PathBuf>{
    paths.into_iter().filter(|p|{
        if let Ok(meta) = fs::metadata(p){
            if let Ok(modified) = meta.modified(){
                let datetime: DateTime<Local> = modified.into();
                return (datetime.year() == y) && (datetime.month() == m);
            }
        }
        false
    }).collect()
}
// filter_by_extension /////////////////////////////////////
fn filter_by_extension(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let allowed_exts = ["png", "jpg", "jpeg", "bmp", "gif", "webp", "avif"];
    paths.into_iter().filter(|p|{
        p.extension()
         .and_then(|e| e.to_str())
         .map(|e| allowed_exts.contains(&e.to_lowercase().as_str()))
         .unwrap_or(false)
    }).collect()
}
// get_month_img_files /////////////////////////////////////
pub fn get_month_img_files(dir: &Path, y: i32, m: u32, ymfilt: bool) -> std::io::Result<Vec<PathBuf>> {
    let files            = list_files(dir)?;
    let filtered_by_data = if ymfilt { filter_by_year_month(files, y, m) } else { files };
    let filtered_files   = filter_by_extension(filtered_by_data);
    // debug
    for f in &filtered_files{ println!("{}", f.display()); }

    return Ok(filtered_files);
}
