use std::{
    fs::{self, File},
    io::{BufReader, Write},
    path::PathBuf,
};

use crate::{
    astra_const::{self, ASTRA_CONFIG_TEMPLATE},
    astra_error::{AstraError, Result},
    astra_git_path::GitPathInfo,
};
use serde::Deserialize;
use serde_json::from_reader;

#[derive(Deserialize, Clone, Debug)]
pub struct AstraConfigUser {
    pub name: String,
    pub email: String,
}

impl AstraConfigUser {
    pub fn get_user(&self) -> Option<(String, String)> {
        if self.name.is_empty() || self.email.is_empty() {
            None
        } else {
            Some((self.name.clone(), self.email.clone()))
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct AstraConfig {
    pub host: String,
    pub owner: String,
    pub id: Option<String>,
    pub user: AstraConfigUser,
}

impl AstraConfig {
    pub fn resolve(&self, info: &GitPathInfo) -> String {
        match &self.id {
            Some(s) => {
                format!("git@{}.{}:{}/{}", s, self.host, self.owner, info.repository)
            }
            None => {
                format!("git@{}:{}/{}", self.host, self.owner, info.repository)
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct AstraConfigList {
    pub list: Vec<AstraConfig>,
}

impl AstraConfigList {
    pub fn select(&self, info: &GitPathInfo) -> Option<&AstraConfig> {
        self.list
            .iter()
            .find(|&f| f.host == info.host && f.owner == info.owner)
    }
}

fn get_config_path() -> Option<PathBuf> {
    let home = match dir::home_dir() {
        Some(s) => s,
        None => return None,
    };

    Some(home.join(format!(
        "{}/{}",
        astra_const::ASTRA_CONFIG_DIRECTORY,
        astra_const::ASTRA_CONFIG_FILE_NAME
    )))
}

#[allow(unused)]
pub fn exist_config() -> bool {
    match get_config_path() {
        Some(s) => s.is_file(),
        None => false,
    }
}

#[allow(unused)]
pub fn create_config() -> bool {
    let home = match dir::home_dir() {
        Some(s) => s,
        None => return false,
    };

    let config_dir = home.join(astra_const::ASTRA_CONFIG_DIRECTORY);
    if !create_config_dir(&config_dir) {
        return false;
    };

    let config_file = config_dir.join(astra_const::ASTRA_CONFIG_FILE_NAME);
    create_config_file(&config_file)
}

fn create_config_dir(config_dir: &PathBuf) -> bool {
    match fs::create_dir_all(config_dir) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("{:?}", e);
            false
        }
    }
}

fn create_config_file(config_file: &PathBuf) -> bool {
    let file = File::create(config_file);

    let mut file = match file {
        Ok(o) => o,
        Err(_) => return false,
    };

    file.write(ASTRA_CONFIG_TEMPLATE.as_bytes()).is_ok()
}

#[allow(unused)]
pub fn load_config() -> Result<AstraConfigList> {
    let file_path = match get_config_path() {
        Some(s) => s,
        None => return Err(AstraError::FailedLoadConfigFile),
    };

    let file = match File::open(file_path) {
        Ok(o) => o,
        Err(_) => return Err(AstraError::FailedLoadConfigFile),
    };

    let reader: BufReader<File> = BufReader::new(file);

    match from_reader::<BufReader<File>, AstraConfigList>(reader) {
        Ok(o) => Ok(o),
        Err(_) => Err(AstraError::FailedParseConfigFile),
    }
}

#[cfg(test)]
mod test_astra_config {
    use crate::astra_git_path::parse_git_path;

    use super::{AstraConfig, AstraConfigList, AstraConfigUser};

    #[test]
    fn test_astra_config_list_select() {
        let sample_github_path: &str = "git@github.com:Creanciel/git-astra.git";

        let config = AstraConfig {
            host: "github.com".to_owned(),
            owner: "Creanciel".to_owned(),
            id: Some("UserAccount".to_owned()),
            user: AstraConfigUser {
                name: "".to_string(),
                email: "".to_owned(),
            },
        };

        let config_dummy = AstraConfig {
            host: "github.com".to_owned(),
            id: Some("hoge".to_owned()),
            owner: "hoge".to_owned(),
            user: AstraConfigUser {
                name: "".to_string(),
                email: "".to_owned(),
            },
        };

        let astra = AstraConfigList {
            list: vec![config, config_dummy],
        };

        let info = parse_git_path(sample_github_path).unwrap();

        println!("{:?}", info);

        println!("{:?}", astra);

        let res = astra.select(&info).unwrap();

        assert_eq!(
            res.resolve(&info),
            "git@UserAccount.github.com:Creanciel/git-astra.git"
        );
    }
}
