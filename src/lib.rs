use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use serde_yaml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub local: String,
    pub remote: String,
}

pub fn config(path: String) -> Config {
    let config_path = Path::new("config.yaml");
    if config_path.exists() {
        println!("find config.yaml");
        println!("-----------------------------------")
    }
     else {
        let mut file = File::create("config.yaml").expect("create config.yaml failed");
        file.write_all(b"local: 0.0.0.0:1259\nremote: 192.168.1.164:1259").expect("write config.yaml failed");
        println!("create config.yaml successed");
        println!("-----------------------------------")
    }
    let file = File::open(path).expect("open config.yaml failed");
    let config: Config = serde_yaml::from_reader(file).expect("read config.yaml failed");
    println!("listen port: {}\nremote port: {}",config.local,config.remote);
    println!("-----------------------------------");
    return Config{local: config.local, remote: config.remote}
}