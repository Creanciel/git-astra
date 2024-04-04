use regex::Regex;

#[derive(Debug)]
pub struct GitPathInfo {
    pub host: String,
    pub owner: String,
    pub repository: String,
}

pub fn parse_git_path(path: &str) -> Option<GitPathInfo> {
    // Support only git@?????????
    let re_git = Regex::new(r"^git@(.+?):(.+?)\/(.+)$").unwrap();
    let c = match re_git.captures(path) {
        Some(s) => s,
        None => return None,
    };

    let host = match c.get(1) {
        Some(o) => o.as_str().to_owned(),
        None => return None,
    };

    let owner = match c.get(2) {
        Some(o) => o.as_str().to_owned(),
        None => return None,
    };

    let repository = match c.get(3) {
        Some(o) => o.as_str().to_owned(),
        None => return None,
    };

    Some(GitPathInfo {
        host,
        owner,
        repository,
    })
}

#[cfg(test)]
mod test_parse_git_path {
    use crate::astra_git_path::parse_git_path;

    #[test]
    fn test_parse_git_path_h1() {
        let sample_github_1_path: &str = "git@github.com:Creanciel/git-astra.git";
        let sample_github_1_host: &str = "github.com";
        let sample_github_1_owner: &str = "Creanciel";
        let sample_github_1_repository: &str = "git-astra.git";

        let info_github_1 = parse_git_path(sample_github_1_path).unwrap();
        assert_eq!(info_github_1.host, sample_github_1_host);
        assert_eq!(info_github_1.owner, sample_github_1_owner);
        assert_eq!(info_github_1.repository, sample_github_1_repository);
    }

    #[test]
    fn test_parse_git_path_h2() {
        let sample_github_2_path: &str = "git@github.com:GITHUB/SampleGitHubPath.git";
        let sample_github_2_host: &str = "github.com";
        let sample_github_2_owner: &str = "GITHUB";
        let sample_github_2_repository: &str = "SampleGitHubPath.git";

        let info_github_2 = parse_git_path(sample_github_2_path).unwrap();
        assert_eq!(info_github_2.host, sample_github_2_host);
        assert_eq!(info_github_2.owner, sample_github_2_owner);
        assert_eq!(info_github_2.repository, sample_github_2_repository);
    }

    #[test]
    fn test_parse_git_path_h3() {
        let sample_github_3_path: &str = "git@github.com:GITHUB/gitpath.git.git";
        let sample_github_3_host: &str = "github.com";
        let sample_github_3_owner: &str = "GITHUB";
        let sample_github_3_repository: &str = "gitpath.git.git";

        let info_github_3 = parse_git_path(sample_github_3_path).unwrap();
        assert_eq!(info_github_3.host, sample_github_3_host);
        assert_eq!(info_github_3.owner, sample_github_3_owner);
        assert_eq!(info_github_3.repository, sample_github_3_repository);
    }

    #[test]
    fn test_parse_git_path_h4() {
        let sample_github_4_path: &str = "git@github.com:GITHUB/git.gitpath.git";
        let sample_github_4_host: &str = "github.com";
        let sample_github_4_owner: &str = "GITHUB";
        let sample_github_4_repository: &str = "git.gitpath.git";

        let info_github_4 = parse_git_path(sample_github_4_path).unwrap();
        assert_eq!(info_github_4.host, sample_github_4_host);
        assert_eq!(info_github_4.owner, sample_github_4_owner);
        assert_eq!(info_github_4.repository, sample_github_4_repository);
    }

    #[test]
    fn test_parse_git_path_l() {
        let sample_gitlab_path: &str = "git@gitlab.com:gitlab/gitlab_project/gitlab_repository.git";
        let sample_gitlab_host: &str = "gitlab.com";
        let sample_gitlab_owner: &str = "gitlab";
        let sample_gitlab_repository: &str = "gitlab_project/gitlab_repository.git";

        let info_gitlab = parse_git_path(sample_gitlab_path).unwrap();
        assert_eq!(info_gitlab.host, sample_gitlab_host);
        assert_eq!(info_gitlab.owner, sample_gitlab_owner);
        assert_eq!(info_gitlab.repository, sample_gitlab_repository);
    }
}
