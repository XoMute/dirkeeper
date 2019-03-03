

use std::env;


#[derive(Debug)]
pub struct CLIArgs{

    pub configpath: String,
    test: bool

}

pub fn get_args() -> CLIArgs{

    let args:Vec<String> = env::args().collect();

    let mut conf_path: String = "".to_string();

    let mut iter = args.iter().skip(1);
    while let Some(v) = iter.next(){
        match v.as_ref() {
            "-c" | "--config" => {
                if let Some(value) = iter.next(){
                    conf_path = value.to_string();
                }else{
                    panic!("Not enough arguments for --config");
                }
            },
            _ => {
                panic!("Unknown option {}",v);
            }
        }
    }


    if conf_path == "" {
        conf_path = get_default_config_path().expect("Can't determine config path, please use --config <path> option")
    }

    let args = CLIArgs{
        configpath: conf_path.to_string(),
        test: false
    };


    return args;

}

fn get_default_config_path() -> Option<String>{
    if let Ok(conf_home) = env::var("XDG_CONFIG_HOME"){
        Debug!("Config home = {}", conf_home);
        let conf_path = conf_home + "/keeperrc";
        return Some(conf_path);
    }else if let Ok(home) = env::var("HOME"){
        Debug!("Home = {}", home);
        let conf_path = home + "/.config/keeperrc";
        return Some(conf_path);
    }
    return None;
}
