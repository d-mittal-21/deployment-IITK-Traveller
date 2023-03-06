// use std::{fs, error::Error};
use regex::Regex;
use lazy_static::lazy_static;

// pub fn take_arguments(args: &Vec<String>) -> Result<String, &str>{
//     if args.len() < 2{
//         return Err("Not enough arguments");
//     }
    
//     let filename: String = args[1].clone();
//     return Ok(filename)

// }

// pub fn get_code(filename : &String) -> Result<String, Box<dyn Error>>{
//     let code = fs::read_to_string(filename)?;

//     return Ok(code);
// }

fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}

fn check_syntax(line:&str) -> bool{
    lazy_static!{
        static ref RE: Regex = Regex::new(r"(.*),[0-9]+,(.*);").unwrap();
    }
    return RE.is_match(line);
}

pub fn parse_code(code: &String) -> Result<Vec<Vec<String>>, i32>{
    let mut parsed: Vec<Vec<String>> = Vec::new();

    let mut counter = 0;
    for line in code.lines(){
        counter += 1;
        if line == "" || line == "\n" {
            continue;
        }

        let no_space = remove_whitespace(line);
        if check_syntax(&no_space) != true {
            return Err(counter);
        }

        let mut temp: Vec<String> = vec![];

        let mut word = String::from("");

        for c in no_space.chars(){
            if c == ',' || c == ';'{
                temp.push(word.clone());
                word.clear();
                continue;
            }
            word.push(c);
        }

        parsed.push(temp);
    }

    return Ok(parsed);
}