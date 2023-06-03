use std::{env::args, fs::DirEntry, path::Path};

use file_path_finder::{check_files, read_dirs, HOME_DIR};

fn main() {
    let args = args().collect::<Vec<String>>();

    let inputs = args.split_first().unwrap().1;
    println!("{:?}", inputs);

    if Path::exists(Path::new(HOME_DIR)) {
        let files = read_dirs(HOME_DIR);
        let new_f: Vec<_> = files
            .into_iter()
            .map(|f| change_to_str(f))
            .map(|f| f.replace("DirEntry(", ""))
            // .map(|f| f.trim_start_matches("\\").to_owned())
            .collect();
        println!("{:#?}", new_f);
        check_files(args[1].as_str(), new_f);
    }

    // println!("{:#?}", args[1]);
}

fn change_to_str(f: DirEntry) -> String {
    format!("{:?}", f)
}
