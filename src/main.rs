mod astra_config;
mod astra_const;
mod astra_error;
mod astra_getopts;
mod astra_git;
mod astra_git_path;

use crate::{
    astra_config::{create_config, exist_config, load_config, AstraConfig},
    astra_const::ASTRA_CONFIG_FILE_NAME,
    astra_error::{AstraError, Result},
    astra_getopts::getopts,
    astra_git::git_clone,
    astra_git_path::parse_git_path,
};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if !exist_config() {
        println!("{} isn't exists", ASTRA_CONFIG_FILE_NAME);

        if create_config() {
            println!("Created");
        } else {
            println!("Failed to create {}", ASTRA_CONFIG_FILE_NAME);
            return Err(astra_error::AstraError::FailedCreateConfig);
        }
    }

    let (mut args, index_list) = getopts(args);

    let mut selected: Option<AstraConfig> = None;

    if index_list.len() == 1 || index_list.len() == 2 {
        let config_list = match load_config() {
            Ok(o) => o,
            Err(_) => return Err(AstraError::FailedLoadConfigFile),
        };

        let path_info = parse_git_path(&args[index_list[0]]);

        if let Some(p) = &path_info {
            if let Some(s) = config_list.select(p) {
                let hoge = s.resolve(p);
                args[index_list[0]] = hoge;
                selected = Some(s.clone());
            }
        }
    }

    git_clone(&args, &selected)
}
