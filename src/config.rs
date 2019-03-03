

use crate::rule::*;
use crate::args;
use std::path::Path;
use std::fs::{File,read_to_string};
use std::io::prelude::*;



pub fn maybe_create_config(path_str: &str) -> std::io::Result<()>{
    let path = Path::new(path_str);

    if path.exists() {
        return Ok(());
    }

    Debug!("Creating config");

    let mut file = File::create(path)?;
    file.write_all(b"#This is dirkeeper config
# \"#\" Means a comment
#Write rules here
")?;

    Ok(())
}


pub fn parse_config(args: &args::CLIArgs) -> Result<Vec<Rule>, String>{
    let mut rules:Vec<Rule> = vec![];

    let conf = match read_to_string(&args.configpath) {
        Ok(data) => data,
        Err(err) => return Err(format!("Cant read config at {} with error {}", args.configpath,err))
    };

    let data:Vec<String> = conf.lines()
        .filter(|line| ! line.trim_start().starts_with('#'))
        .filter(|line| line.len() > 0)
        .map(|line| line.to_string())
        .collect();

    Debug!("Config without comments - {:?}", data);


    let mut lines = data.iter().enumerate();
    while let Some((index, line)) = lines.next(){

    }




    Ok(rules)
}
