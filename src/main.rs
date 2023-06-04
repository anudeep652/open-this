use std::env::args;

use std::path::Path;

use open_this::{read_dirs, search_all_dirs, HOME_DIR};

fn main() {
    let args = args().collect::<Vec<String>>();

    if Path::exists(Path::new(unsafe { HOME_DIR })) {
        let files = read_dirs(unsafe { HOME_DIR });

        search_all_dirs(files, &args)
    }
}
