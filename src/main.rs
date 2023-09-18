use std::env::var;
use std::fmt::format;
use std::fs::{read, File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_name = File::open(&args[1]).expect("Unable to open file");
    let reader = BufReader::new(file_name);
    let mut var_names = Vec::new();
    let mut fixed_point = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(".rs:") {
            fixed_point.push(line);
        } else if line.contains(".rs-") {
            var_names.push(line);
        }
    }
    let mut fin = Vec::new();
    let mut name_final = Vec::new();
    for fixed in &fixed_point {
        let new_string: Vec<&str> = fixed.split('(').collect();
        let final_val: Vec<&str> = new_string[1].split(',').collect();
        let final_val = final_val[0].replace('_', "");
        let final_val: Vec<&str> = final_val.split(')').collect();
        // let final_val = final_val.replace(']', "");
        // println!("{}", final_val);
        fin.push(final_val[0].parse::<u32>().unwrap());
    }
    for name in &var_names {
        let mut first_byte = name.find("pub").unwrap();
        let found: Vec<&str> = name.split("pub").collect();
        let indexed_name: Vec<&str> = found[0].rsplit('/').collect();
        let mut file_name: String =
            indexed_name[0].split('.').collect::<Vec<&str>>()[0].to_uppercase();
        let final_name: Vec<&str> = found[1].split(':').collect();
        let final_text = file_name + "_" + final_name[0].trim();
        name_final.push(final_text);
    }
    let mut line_num: usize;
    println!("{}", name_final.len());
    println!("{}", fin.len());
    if name_final.len() >= fin.len() {
        line_num = name_final.len();
    } else {
        line_num = fin.len();
    }
    let mut ts_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("test2.ts")
        .unwrap();
    for num in 0..line_num {
        let output = format!(
            "export const {}= {};\n",
            name_final[num].to_uppercase(),
            fin[num]
        );
        ts_file.write(output.as_bytes()).expect("can't write");
    }
}
