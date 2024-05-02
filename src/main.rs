extern crate ansi_term;
extern crate chrono;

use ansi_term::Colour;
use chrono::{Local, Datelike};
use std::io::Write;
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
        _ => println!("Error to switch mode")
    }
}

fn log_mode(match_str: &String) -> u64 {
    match match_str as &str {
        "nolg" => 0xA,      // NorthLog格式
        "raw" => 0xB,       // Raw格式
        _ => 0xFF           // 其他情况
    }
}

fn makelog_nolg(content: &String){
    let now = Local::now();
    let time_to_write = now.year().to_string() + "0" + &now.month().to_string()+ "0" + &now.day().to_string();
    
    let file_head = String::from("nolg-");
    let file_ending = String::from("-report.log");

    let final_filename = file_head + &time_to_write + &file_ending;

    let mut file = std::fs::File::create(final_filename).expect("Failed to create file.");
    println!("{:?}",file);

    file.write_all(content.as_bytes()).expect("写入失败");
    println!("content has been written to the file.");
}

fn makelog_raw(content: &String){
    let mut file = std::fs::File::create("logfile.txt").expect("failed to create");
    file.write_all(content.as_bytes()).expect("write failed");
    println!("content has been written to the file.");
}
