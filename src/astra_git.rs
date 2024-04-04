use crate::{
    astra_config::AstraConfig,
    astra_error::{AstraError, Result},
};
use std::process::{Command, Stdio};

pub fn git_clone(args: &Vec<String>, config: &Option<AstraConfig>) -> Result<()> {
    let mut config_user: Option<(String, String)> = None;

    if let Some(c) = config {
        if let Some(s) = c.user.get_user() {
            config_user = Some(s);
        }
    }

    let mut process = Command::new("git");
    let process = process.arg("clone");

    if let Some(c) = config_user {
        process
            .arg("--config")
            .arg(format!("user.name={}", c.0))
            .arg("--config")
            .arg(format!("user.email={}", c.1));
    };

    let process = process.args(args);

    let process = process
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    let mut process = match process {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{:?}", e);
            return Err(AstraError::FailedCommandGitClone);
        }
    };

    let _ = match process.wait() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{:?}", e);
            return Err(AstraError::FailedCommandGitClone);
        }
    };

    Ok(())
}

#[cfg(test)]
mod test_astra_git {
    use crate::astra_config::{AstraConfig, AstraConfigUser};

    use super::git_clone;

    #[test]
    fn test_astra_git_git_clone() {
        let config = &Some(AstraConfig {
            host: "github.com".to_owned(),
            owner: "Creanciel".to_owned(),
            id: Some("creanciel".to_owned()),
            user: AstraConfigUser {
                name: "Creanciel".to_owned(),
                email: "creanciel@example.com".to_owned(),
            },
        });
        let _ = git_clone(
            &vec!["git@github.com:Creanciel/git-astra.git".to_owned()],
            config,
        );
    }
}
