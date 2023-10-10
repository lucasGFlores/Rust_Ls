// use std::io;
// use std::process;
extern crate colored;
// use clap::Args;
use clap::Parser;
// use std::cmp::Ordering;
use colored::*;
use std::env;
use std::fs;
use std::fs::DirEntry;
// use  std::cmp::Ordering;

#[derive(Parser)]
struct Cli {
    path: String,
}
enum CommandUsage {
    Params,
    PathAndParams,
    Path,
    None,
}
#[derive(Debug)]
enum DataParams {
    //this is for show hidden data of the arquives
    None,
    DataFlood, // show all data of the arquives
} //make impl for each one of this
impl DataParams {
    fn show_data(&self,meta_data : &DirEntry) -> (i8,String) {
        match self {
            DataParams::None => (0,String::from("")),
            DataParams::DataFlood => (2, format!("{:?}", meta_data.metadata().unwrap())),
        }
    }
}
enum FilterParams {
    // arquives filter
    Hidden, // show hidden arquives
    None,   // dont do anything
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let path = match type_usage(&args) {
        CommandUsage::None | CommandUsage::Params => ".".to_string(),
        CommandUsage::Path | CommandUsage::PathAndParams => args[1].clone(),
    };

    /* this block of code is for split the to types of params exist
     */
    let params: (Vec<DataParams>, FilterParams) = match type_usage(&args) {
        CommandUsage::None | CommandUsage::Path => (vec![DataParams::None], FilterParams::None),

        CommandUsage::Params | CommandUsage::PathAndParams => {
            let mut dataparams: Vec<DataParams> = Vec::new();
            let mut filterparams: FilterParams = FilterParams::None;
            for i in 0..args.len() {
                if args[i].contains("-") {
                    match args[i].as_str() {
                        //list of params

                        // I dont know if just the last Filter param set matters to de the code
                        // in future I can think if it's a huge problem
                        "-a" => filterparams = FilterParams::Hidden,
                        "-df" => dataparams.push(DataParams::DataFlood),
                        _ => panic!("invalid param: {}", args[i].as_str()),
                    }
                }
            }

            if dataparams.len() == 0 {
                dataparams.push(DataParams::None);
            }
            (dataparams, filterparams)
        }
    };

    ls(&path, params);
}

fn ls(path: &String, params: (Vec<DataParams>, FilterParams)) {
    match fs::read_dir(path) {
        Ok(dirs) => {
            for dir in dirs {
                match dir {
                    Ok(dir_info) => {
                        match params.1 {
                            FilterParams::Hidden => {}
                            FilterParams::None => {
                                if dir_info.file_name().to_string_lossy().starts_with(".") {
                                    continue;
                                }
                            }
                        }
                         print!("----\n{:?}  ",params.0);
                        // if you will goin to make a files filter, add a new arm match
                        // if not, you gonna make a if in the print method

                        let mut response : Vec<(i8,String)> = Vec::new(); //this will join the data of arquive to show in terminal
                        response.push((0,dir_info.file_name().to_string_lossy().to_string()));
                        // another Idea is to make a long print! with all the data is showin
                        //like print"("{} {}",dir_info.filename(),permissions)
                        /*if I the code receive a param to see the permissions, the var permissions will have
                        the String of file permissions, if not, the var permission will be " " */

                        // another way is make a implement each one of the params
                        // returning the value and the position in the print
                        //I think this will gonna be bether using a vector of DataParams and use for

                        //put this if in the implement DataParams????
                        let file_name = dir_info.file_name().to_string_lossy().to_string();
                        if dir_info.metadata().unwrap().is_dir() {
                           file_name.blue();
                        }
                        for param in &params.0 {
                            let (pos,data) = param.show_data(&dir_info);
                            response.push((pos,data));
                           response.sort_by_key(|k| k.0)
                        }
                        
                        print!("{:?}\n",response);
                    }
                    Err(_) => panic!("OOOOO SHIT"),
                }
            }
        }
        Err(e) => panic!("{:?}", e),
    }
}
fn type_usage(args: &Vec<String>) -> CommandUsage {
    if args.len() == 1 {
        return CommandUsage::None;
    }
    match args[1].to_string().contains("-") {
        true => CommandUsage::Params,
        false => {
            if args.len() > 2 && args[2].to_string().contains("-") {
                CommandUsage::PathAndParams
            } else {
                CommandUsage::Path
            }
        }
    }
}
// fn cat (path: &String) {
// match fs::read(path) {
//     Ok(bytes) => {
//         print!("{:#?}",String::from_utf8(bytes).unwrap())
//     },
//     Err(err) => panic!("OO SHIT\n{}",err),
// }
// }
