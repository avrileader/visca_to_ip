use serde::Deserialize;
use std::fs::File;
use serde_yaml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub local: String,
    pub remote: String,
}
pub fn config(path: String) -> Config {
    let file = File::open(path).expect("open file failed");
    let config: Config = serde_yaml::from_reader(file).expect("read railed");
    print!("listen port: {} \n",config.local);
    print!("remote port: {} \n",config.remote);
    return Config{local: config.local, remote: config.remote};
}