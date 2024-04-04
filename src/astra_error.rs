#[allow(unused)]
#[derive(Debug)]
pub enum AstraError {
    FailedLoadConfigFile,
    FailedParseConfigFile,
    SupportOnlySshClone,
    FailedParseGitPath,
    UnresolveGitPath,
    FailedExecClone,
    FailedCommandGitClone,
    FailedCreateConfig,
    FailedGetCloneDirectory,
    FailedGetUser,
}

#[allow(unused)]
pub type Result<T> = std::result::Result<T, AstraError>;
