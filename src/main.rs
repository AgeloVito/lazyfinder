use std::{num::ParseIntError, fs::DirEntry};
use std::fs;
use walkdir::WalkDir;
use colored::Colorize;

use clap::Parser;

/// 遍历目标目录中包含指定关键字的文件，并从匹配到的文件中匹配特定字符串所在行
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// 目标目录
    #[clap(short, long)]
    dir: String,

    /// 指定文件名称关键字
    #[clap(short, long)]
    pre: String,

    /// 指定文件内容关键字
    #[clap(short, long)]
    keys: String,
}

fn main() {
    let args = Args::parse();

    let dir  = args.dir;
    let pre:Vec<&str> = args.pre.as_str().split(",").collect();
    let keys:Vec<&str> = args.keys.as_str().split(",").collect();
    let f_name = WalkDir::new(dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir());
    
    for i in f_name{
        for j in pre.clone().into_iter(){
            if i.file_name().to_string_lossy().contains(j){
                println!("{} : {} ","path".blue(),i.path().to_string_lossy().to_string());
                
                for k in keys.clone().into_iter(){

                    match  fs::read_to_string(i.path().to_string_lossy().to_string()){
                            Ok(contents)=>{
                                let mut count = 0;
                                for l in contents.lines(){
                                    count+=1;
                                    if l.contains(k){
                                        println!("\t{} {}: {}","line".red(),count.to_string().green(),l)
                                    }
                                }
                            }
                            Err(err)=>{println!("{}",&err)}
                    };
                    
                }
                
            }
        }
    }
}