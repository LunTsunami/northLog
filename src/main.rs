mod compute;
mod utility;

use chrono::{Local, Datelike};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::string::ToString;
use std::env;

use compute::compute_fn;

fn main() {
    let args:Vec<String> = env::args().collect();
    
    if args[2..].is_empty() {
        utility::function::panic_console(3, "Empty Args, exiting!".to_string());
    }

    let option = &args[1];
    let content = &args[2];
    let filetype_input = &args[3];
    
    let filetype_out = match filetype_input as &str {
        "1"   => "log",
        "2"   => "txt",
        "def" => "log",
         _    => "log"
    };
    println!("norlg => option: {} content: {}",option, content);

    let mode = log_mode(option);
    match mode{
        0xA  => makelog_nolg(content,filetype_out),
        0xB  => makelog_raw(content),
        0xC  => makelog_nolgq(content,filetype_out),
        0xD  => makelog_rawq(content),
        0xE  => makelog_nolg(content, filetype_out),
        0xF  => makelog_rawts(content),
        _ => utility::function::panic_console(3, "Error to switch mode".to_string())
    }
}

/// # logmode
/// 用于确定Log输出的模式  
/// 包含两个模式：
/// * nolg  模式
/// * raw   模式
/// * nolgq 模式
/// * rawq  模式
/// * rawts 模式
fn log_mode(match_str: &String) -> u64 {
    match match_str as &str {
        "nolg"  => 0xA,         // NorthLog格式
        "raw"   => 0xB,         // Raw格式
        "nolgq" => 0xC,         // 安静NorthLog格式
        "rawq"  => 0xD,         // 安静Raw格式
        "def"   => 0xE,         // 默认情况（NorthLog格式） def => default
        "rawts" => 0xF,         // Raw TimeStamp模式
        _ => 0xFF               // 其他情况
    }
}

fn makelog_nolg(content: &String, filetype:&str){
    let now = Local::now();
    let time_to_write = now.year().to_string() + "0" + &now.month().to_string()+ "0" + &now.day().to_string();
    
    let file_head = String::from("nolg-");
    let file_ending = String::from("-report.") + filetype;

    let final_filename = file_head + &time_to_write + &file_ending;

    let file_path = Path::new(final_filename.as_str());
    if file_path.exists() {
        println!("文件存在");
        let mut file_a = OpenOptions::new().append(true).open(final_filename).expect("error to open file");
        println!("{:?}",file_a);
        file_a.write_all("\n".as_bytes()).expect("Error to Open");
        file_a.write_all(content.as_bytes()).expect("Error To Open");
        println!("content has been written to the file.");
    }
    else {
        let hashvalue = compute_fn::gen_hash_from(content);

        let mut file = std::fs::File::create(final_filename).expect("Failed to create file.");
        println!("{:?}",file);
        file.write_all("============================\nHASH VALUE HEAD BEGIN: \n".as_bytes()).expect("Failed to open");
        file.write_all(hashvalue.to_string().as_bytes()).expect("Failed to write");
        file.write_all("\nHASH VALUE HEAD END. \n============================\n".as_bytes()).expect("Failed to open");

        file.write_all(content.as_bytes()).expect("写入失败");
        println!("content has been written to the file.");
    }
    
}

fn makelog_raw(content: &String){
    let now = Local::now();
    let mut file = std::fs::File::create(String::from("logfile-raw.txt") + &now.timestamp().to_string()).expect("failed to create");
    file.write_all(content.as_bytes()).expect("write failed");
    println!("content has been written to the file.");
}

fn makelog_nolgq(content: &String, filetype:&str){
    let now = Local::now();
    let time_to_write = now.year().to_string() + "0" + &now.month().to_string()+ "0" + &now.day().to_string();
    
    let file_head = String::from("nolg-");
    let file_ending = String::from("-report.") + filetype;

    let final_filename = file_head + &time_to_write + &file_ending;

    let file_path = Path::new(final_filename.as_str());
    if file_path.exists() {
        let mut file_a = OpenOptions::new().append(true).open(final_filename).expect("error to open file");
        file_a.write_all("\n".as_bytes()).expect("Error to Open");
        file_a.write_all(content.as_bytes()).expect("Error To Open");
    }
    else {
        let mut file = std::fs::File::create(final_filename).expect("Failed to create file.");
        file.write_all("\n".as_bytes()).expect("Error to Open");
        file.write_all(content.as_bytes()).expect("写入失败");
    }
    
}

fn makelog_rawq(content: &String){
    let now = Local::now();
    let mut file = std::fs::File::create(String::from("logfile-raw.txt") + &now.timestamp().to_string()).expect("failed to create");
    file.write_all(content.as_bytes()).expect("write failed");
    println!("content has been written to the file.");
}

fn makelog_rawts(content: &String){
    let final_filename = "log-RAW_TIMESTAMP-".to_string() + &utility::time_utility::timestamp_get().to_string() + ".log";
    utility::time_utility::timestamp_output();
    let mut file = std::fs::File::create(String::from(final_filename)).expect("failed to create");
    file.write_all(content.as_bytes()).expect("write failed");
    println!("content has been written to the file.");
}