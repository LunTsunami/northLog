extern crate ansi_term;
extern crate chrono;

use ansi_term::Colour;
use chrono::{Local, Datelike};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::string::ToString;
use std::env;


fn main() {
    let args:Vec<String> = env::args().collect();
    
    if args[1..].is_empty() {
        println!("{}",Colour::Red.paint("Error: Not A parameter."));
    }

    let option = &args[1];
    let content = &args[2];

    println!("参数输入 => option: {} content: {}",option, content);

    let mode = log_mode(option);
    match mode{
        0xA => makelog_nolg(content),
        0xB => makelog_raw(content),
        0xC => makelog_nolgq(content),
        0xD => makelog_rawq(content),
        _ => println!("Error to switch mode")
    }
}

/// # logmode
/// 用于确定Log输出的模式  
/// 包含两个模式：
/// * nolg 模式
/// * raw  模式
fn log_mode(match_str: &String) -> u64 {
    match match_str as &str {
        "nolg"  => 0xA,         // NorthLog格式
        "raw"   => 0xB,         // Raw格式
        "nolgq" => 0xC,         // 安静NorthLog格式
        "rawq"  => 0xD,         // 安静Raw格式
        _ => 0xFF               // 其他情况
    }
}

fn makelog_nolg(content: &String){
    let now = Local::now();
    let time_to_write = now.year().to_string() + "0" + &now.month().to_string()+ "0" + &now.day().to_string();
    
    let file_head = String::from("nolg-");
    let file_ending = String::from("-report.log");

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
        let mut file = std::fs::File::create(final_filename).expect("Failed to create file.");
        println!("{:?}",file);
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

fn makelog_nolgq(content: &String){
    let now = Local::now();
    let time_to_write = now.year().to_string() + "0" + &now.month().to_string()+ "0" + &now.day().to_string();
    
    let file_head = String::from("nolg-");
    let file_ending = String::from("-report.log");

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