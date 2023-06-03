use std::{env::args, path::Path};

use file_path_finder::{read_dirs, search_all_dirs, HOME_DIR, NEXT_DIR_PATH};

fn main() {
    let args = args().collect::<Vec<String>>();

    let inputs = args.split_first().unwrap().1;
    println!("{:?}", inputs);

    if Path::exists(Path::new(HOME_DIR)) {
        let files = read_dirs(HOME_DIR);
        unsafe { NEXT_DIR_PATH.push(HOME_DIR.to_owned()) };

        search_all_dirs(files, &args)
    }
}
