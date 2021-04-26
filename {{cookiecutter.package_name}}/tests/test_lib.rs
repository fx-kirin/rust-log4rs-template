extern crate {{cookiecutter.package_name}};

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    fn find_log_config_directory<'a>(exe_path: &'a PathBuf) -> &'a Path {
        let mut exe_path: &std::path::Path = exe_path;
        loop {
            exe_path = match exe_path.parent() {
                Some(exe_path) => exe_path,
                None => panic!("failed to get parent path"),
            };
            if exe_path.join("config").exists() {
                break;
            }
        }
        exe_path
    }

    #[test]
    fn it_works() {
        let exe_path = match std::env::current_exe() {
            Ok(exe_path) => exe_path,
            Err(e) => panic!("failed to get current exe path: {}", e),
        };
        let exe_path = find_log_config_directory(&exe_path);
        if cfg!(debug_assertions) {
            log4rs::init_file(
                exe_path.join("config/debug/log4rs.yaml"),
                Default::default(),
            )
            .unwrap();
        } else {
            log4rs::init_file(
                exe_path.join("config/release/log4rs.yaml"),
                Default::default(),
            )
            .unwrap();
        }

        assert_eq!(2 + 2, 4);
    }
}
