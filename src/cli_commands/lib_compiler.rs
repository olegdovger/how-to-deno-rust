use crate::cli_commands::CLICommandInterface;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{env};

pub mod build;

#[derive(Debug, Default)]
pub struct Compiler {}

pub fn is_changes_detected(path_buf: &PathBuf) -> bool {
    match path_buf.is_dir() {
        true => false,
        false => {
            let ignored_paths = ["\\pkg", "\\target", "\\Cargo.lock"];
            let path_str = path_buf.to_str().unwrap();

            let result_with_ignored_paths = ignored_paths
                .iter()
                .filter(|s| path_str.to_string().contains(&s[..]))
                .collect::<Vec<_>>();

            result_with_ignored_paths.len() == 0
        }
    }
}

pub fn get_lib_folder_to_build(path: &Path) -> String {
    let mut lib_dir = env::current_dir().unwrap();
    lib_dir.push("lib");

    let lib_relative_path = path.strip_prefix(lib_dir).unwrap();
    let lib_folder = Path::new(lib_relative_path).iter().collect::<Vec<_>>()[0];
    let mut lib_path_to_build = String::from("lib");

    lib_path_to_build.push_str("/");
    lib_path_to_build.push_str(&lib_folder.to_str().unwrap());

    lib_path_to_build
}

pub fn get_relative_path(path: &Path) -> &str {
    let mut lib_dir = env::current_dir().unwrap();
    lib_dir.push("lib");

    let lib_relative_path = path.strip_prefix(lib_dir).unwrap();

    lib_relative_path.to_str().unwrap()
}

fn process_changes(path_buf: &PathBuf, description: &str) {
    if is_changes_detected(&path_buf) {
        let folder = get_lib_folder_to_build(path_buf.as_path());
        let relative_path = get_relative_path(path_buf.as_path());

        print!(" Changes in library {:?}: {:?}", folder, relative_path);
        print!(" [{}]\n", description);

        build::build(&folder);
    }
}

impl CLICommandInterface for Compiler {
    fn run() {
        print!("\n Start \"serve\" command\n");

        let (tx, rx) = channel();

        let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

        println!("\n Watch \"lib\" folder \n");

        watcher.watch("lib", RecursiveMode::Recursive).unwrap();

        loop {
            match rx.recv() {
                Ok(DebouncedEvent::Write(path_buf)) => process_changes(&path_buf, "change"),
                Ok(DebouncedEvent::Create(path_buf)) => process_changes(&path_buf, "create"),
                Ok(DebouncedEvent::Remove(path_buf)) => process_changes(&path_buf, "remove"),
                Ok(DebouncedEvent::Rename(old_path_buf, new_path_buf)) => process_changes(
                    &old_path_buf,
                    &format!(
                        "rename: {:?} --> {:?}",
                        get_relative_path(&old_path_buf),
                        get_relative_path(&new_path_buf)
                    ),
                ),
                Err(_e) => break,
                _ => continue,
            }
        }
    }
}
