use std::{
    fs::{self, DirEntry},
    io::{self},
    path::{Path, PathBuf},
};

pub const HOME_DIR: &str = "/home/anudeep";

pub fn s_dir(name: &str) -> bool {
    let path = PathBuf::from(name);
    path.is_dir()
}

pub fn read_dirs(path: &str) -> Vec<DirEntry> {
    let rs = fs::read_dir(Path::new(path));

    rs.unwrap()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|f| f.unwrap())
        .collect::<Vec<_>>()
}

pub fn check_files(file_name: &str, searched: Vec<String>) {
    searched.into_iter().for_each(|f| {
        if f.contains(file_name) {
            println!("file found at {}", f);
            println!("Do you want to open it? (y/n)");

            let typed = get_input();
            println!("{:?}", typed.len());

            let (first, _) = f.split_at(1).1.split_at(f.len() - 3);

            println!("{:?}", typed);
            match typed.as_str() {
                "y\n" => {
                    println!("Opening file");
                    open_file(first)
                }
                "n\n" => {
                    println!("Ok, Goodbye");
                }

                &_ => {}
            }
        }
    });
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read ");

    if input.as_str().trim().len() > 1 {
        println!("Please type y or n");
    }

    input
}

fn open_file(path: &str) {
    match open::that(Path::new(path)) {
        Ok(_) => println!("Opened"),
        Err(e) => println!("Error: {}", e),
    }
}
