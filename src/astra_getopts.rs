use regex::Regex;

pub fn getopts(args: Vec<String>) -> (Vec<String>, Vec<usize>) {
    let mut new_args = args.clone();
    if new_args.first().is_some() {
        new_args.remove(0);
    }

    let mut index_list = Vec::<usize>::new();

    let mut counter = 0;

    #[allow(clippy::needless_range_loop)]
    while counter < new_args.len() {
        match arg_shift(&new_args[counter]) {
            Some(s) => counter += s,
            None => {
                index_list.push(counter);
            }
        }
        counter += 1;
    }

    (new_args, index_list)
}

fn arg_shift(arg: &str) -> Option<usize> {
    let re_uniarg = Regex::new(r"^((-l)|(-s)|(--no-hardlinks)|(-q)|(-n)|(--bare)|(--mirror)|(--dissociate)|(--single-branch)|(--no-single-branch)|(--no-tags)|(--shallow-submodules)|(--no-shallow-submodules)|(--remote-submodules)|(--no-remote-submodules)|(--sparse)|(--reject-shallow)|(--no-reject-shallow)|(--also-filter-submodules)|(--)|(--recurse-submodules)|(--recurse-submodules=.+)|(--filter=.+)|(--template=.+)|(--recurse-submodules)|(--recurse-submodules=.*)|(--filter=.*)|(--template=.*))$").unwrap();
    if re_uniarg.captures(arg).is_some() {
        return Some(0);
    }

    let re_multiarg =
        Regex::new(r"^((-o)|(-b)|(-u)|(--reference)|(--separate-git-dir)|(--depth)|(--jobs))$")
            .unwrap();
    if re_multiarg.captures(arg).is_some() {
        return Some(1);
    }

    None
}

#[cfg(test)]
mod test_astra_getopts {
    use crate::astra_getopts::getopts;

    #[test]
    fn test_astra_getopts_1() {
        let args = vec![
            "argfilepath".to_owned(),
            "-l".to_owned(),
            "git@aaa".to_owned(),
            "-s".to_owned(),
            "git@aaa".to_owned(),
        ];

        assert_eq!(vec![1, 3], getopts(args).1);
    }

    #[test]
    fn test_astra_getopts_2() {
        let args = vec![
            "argfilepath".to_owned(),
            "--template=aaaa".to_owned(),
            "git@aaa".to_owned(),
            "-u".to_owned(),
            "foobar".to_owned(),
        ];

        assert_eq!(vec![1], getopts(args).1);
    }
}
