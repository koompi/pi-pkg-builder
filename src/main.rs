#![allow(
    non_snake_case,
    unused_imports,
    unused_assignments,
    unused_variables,
    dead_code
)]

mod interfaces;
mod schemas;

use rayon::prelude::*;
use schemas::{desc_file::App, store::Store};
use std::{
    fs,
    io::{stderr, Write},
    process::Command,
    thread,
    time::{Duration, Instant},
};

fn main() {
    let start = Instant::now();
    let mut db_files: Vec<String> = Vec::new();
    let paths = fs::read_dir("/var/lib/pacman/sync/").unwrap();
    for path in paths {
        let file_path = format!("{}", path.unwrap().path().display());
        db_files.push(file_path);
    }

    db_files.par_iter().for_each(|f| extractor(f, "work"));

    // build individual json file
    // build_json_files(true, "work");

    // build store file
    let apps = build_store_file("work");
    let app_store = Store::new(apps);
    app_store.export().unwrap_or(());

    // set stop time
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

fn extractor(data_path: &str, output_dir: &str) {
    println!("Extracting: {}", &data_path);
    let unzip = Command::new("tar")
        .arg("-xf")
        .arg(data_path)
        .arg("-C")
        .arg(output_dir)
        .output()
        .expect("failed to execute process");
    stderr().write_all(&unzip.stderr).unwrap_or(());
}

fn build_json_files(threaded: bool, data_path: &str) {
    /*
     * Generate json description file
     */
    let paths = fs::read_dir(data_path).unwrap();

    match threaded {
        true => {
            /*
             * Concurrent
             */
            let mut all_file: Vec<String> = Vec::new();
            for path in paths {
                let file_path: String = format!("{}/desc", path.unwrap().path().display());
                all_file.push(file_path.clone());
            }

            all_file.par_iter().for_each(|n| {
                let mut package_info: App = App::new();
                package_info.import(n).export().unwrap_or(());
            });
        }
        false => {
            /*
             * Non-concurrent
             */
            for path in paths {
                let file_path = &format!("{}/desc", path.unwrap().path().display());
                let mut package_info: App = App::new();
                package_info.import(file_path).export().unwrap_or(());
            }
        }
    }
}

fn build_store_file(data_path: &str) -> Vec<App> {
    /*
     * Generate json description file
     */
    let paths = fs::read_dir(data_path).unwrap();
    let mut all_apps: Vec<App> = Vec::new();

    for path in paths {
        let file_path = &format!("{}/desc", path.unwrap().path().display());
        let mut package_info: App = App::new();
        let app = package_info.import(file_path);
        all_apps.push(app);
    }

    all_apps
}
