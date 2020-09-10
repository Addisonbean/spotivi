use std::{
    io::{BufReader, BufRead, Read},
    path::PathBuf,
};
use std::fs::File;

use anyhow::Result;
use regex::Regex;
use xdg::BaseDirectories;

use crate::keybindings::{self, KeyBindings};

pub struct Config {
    pub api_client_id: String,
    pub api_client_secret: String,
    pub redirect_uri: String,

    pub keybindings: KeyBindings,
    pub cache_path: PathBuf,
    xdg_dirs: BaseDirectories,
}

impl Config {
    pub fn new() -> Result<Config> {
        let mut conf = Config::default()?;

        let config_path = conf.xdg_dirs.place_config_file("config")?;
        if config_path.exists() {
            let mut config_file = File::open(config_path)?;
            conf.read_config_file(&mut config_file)?;
        }

        keybindings::default_keybindings(&mut conf.keybindings);

        Ok(conf)
    }

    // Not using std::default::Default because Config::default can fail
    fn default() -> Result<Config> {
        let xdg_dirs = BaseDirectories::with_prefix("spotivi")?;
        Ok(Config {
            api_client_id: String::new(),
            api_client_secret: String::new(),
            redirect_uri: "http://localhost:8888/callback".to_owned(),
            keybindings: KeyBindings::new(),
            cache_path: xdg_dirs.place_cache_file("api_auth.json")?,
            xdg_dirs,
        })
    }

    fn read_config_file<R: Read>(&mut self, config: &mut R) -> Result<()> {
        let re = Regex::new(r"^\s*([^ =]+)\s*=\s*([^ =]+)\s*$").unwrap();
        for line in BufReader::new(config).lines() {
            match re.captures(&line?[..]) {
                Some(caps) => {
                    let key = &caps[1];
                    let value = &caps[2];
                    match key {
                        "api_client_id" => self.api_client_id = value.to_owned(),
                        "api_client_secret" => self.api_client_secret = value.to_owned(),
                        "redirect_uri" => self.redirect_uri = value.to_owned(),
                        _ => todo!(),
                    }
                }
                None => todo!(),
            }
        }
        Ok(())
    }
}
