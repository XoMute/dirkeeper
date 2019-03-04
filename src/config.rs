

use crate::rule::*;
use crate::args;
use std::path::Path;
use std::fs::{File,read_to_string};
use std::io::prelude::*;


#[derive(Debug)]
pub struct Config{
    pub rules : Vec<Rule>
}

pub fn maybe_create_config(args: &args::CLIArgs) -> std::io::Result<()>{
    if args.conf_manual {
        return Ok(());
    }

    let path = Path::new(&args.configpath);

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


pub fn parse_config(args: &args::CLIArgs) -> Result<Config, String>{
    let mut conf = Config{rules:vec![]};

    let confstr = match read_to_string(&args.configpath) {
        Ok(data) => data,
        Err(err) => return Err(format!("Cant read config at {} with error {}", args.configpath,err))
    };

    let data:Vec<String> = confstr.lines()
        .filter(|line| ! line.trim_start().starts_with('#'))
        .filter(|line| line.len() > 0)
        .map(|line| line.to_string())
        .collect();

    Debug!("Config without comments - {:?}", data);


    let mut lines = data.iter().enumerate();
    let mut ctx = Context{dir: "".to_string(), file: "".to_string()};
    while let Some((index, line)) = lines.next(){
        let option = parse_rule(&mut ctx, line.to_string(), index as i32)?;
        if let Some(rule) = option{
            conf.rules.push(rule);
        }

    }

    Ok(conf)
}

struct Context{
    dir: String,
    file: String,
}

fn parse_rule(ctx: & mut Context, rulestr: String, index: i32) -> Result<Option<Rule>, String>{
    let cmd: Vec<&str> = rulestr.split(" ").collect();

    match cmd[0] {
        "dir" => {
            if cmd.len() < 2{
                return Err(format!("Need more arguments at line \"{}\"",rulestr));
            }
            ctx.dir = cmd[1].to_string();
            return Ok(None);
        },
        "match" => {
            if cmd.len() < 2{
                return Err(format!("Need more arguments at line \"{}\"",rulestr));
            }
            if ctx.dir == "" {
                return Err(format!("Please specify dir before \"{}\"", rulestr));
            }else{
                ctx.file = cmd[1].to_string();
                return Ok(None);
            }
        },
        "move" => {
            if ctx.dir == "" {
                return Err(format!("Please specify dir before \"{}\"", rulestr));
            }else if ctx.file == ""{
                return Err(format!("Please specify file before \"{}\"", rulestr));
            }
            let rule = Rule{dir:ctx.dir.clone(), file:ctx.file.clone(), action:"move".to_string()};
            ctx.file = "".to_string();
            return Ok(Some(rule));
        },
        _ => { return Err(format!("Unknown command in config at line \"{}\"",rulestr)); }


    }


}
