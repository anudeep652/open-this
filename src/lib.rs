use std::{
    fs::{self, DirEntry},
    io::{self},
    path::{Path, PathBuf},
    process::Command,
};

pub static mut HOME_DIR: &str = "/home/anudeep";
pub static mut NEXT_DIR_PATH: Vec<String> = vec![];
const VS_CODE: &str = "code";

fn is_dir(name: &str) -> bool {
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

#[allow(clippy::single_char_pattern)]
#[allow(clippy::toplevel_ref_arg)]
fn check_files(file_args: &Vec<String>, searched: Vec<String>) {
    let mut found_file = false;
    searched
        .iter()
        .map(|f| return_splitted_path(f.to_owned()))
        .any(|f| search(f, file_args[1].as_str(), &mut found_file));

    // println!("{:#?}Here", found_file);

    if !found_file {
        if !searched.is_empty() && !is_dir(searched[0].as_str()) {
            unsafe {
                NEXT_DIR_PATH.clear();
                NEXT_DIR_PATH.push(format!("{}/", HOME_DIR))
            };
        }
        // println!("Here too");
        searched
            .iter()
            .map(|f| return_splitted_path(f.to_owned()))
            .for_each(|f| {
                let ref next_dir = f.split("/").map(|f| f.to_owned()).collect::<Vec<String>>();
                let length = next_dir.len();

                unsafe { NEXT_DIR_PATH.push(format!("{}/", next_dir[length - 1])) }
                // println!("{:#?}", Path::new(unsafe { &NEXT_DIR_PATH.join("") }));
                if Path::exists(Path::new(unsafe { &NEXT_DIR_PATH.join("") }))
                    && is_dir(unsafe { &NEXT_DIR_PATH.join("") })
                {
                    let files = read_dirs(unsafe { &NEXT_DIR_PATH.join("") });
                    search_all_dirs(files, file_args)
                } else {
                    unsafe {
                        NEXT_DIR_PATH.clear();
                        NEXT_DIR_PATH.push(format!("{}/", HOME_DIR))
                    };
                }
            })
    }
}

pub fn search_all_dirs(files: Vec<DirEntry>, args: &Vec<String>) {
    let new_f: Vec<_> = files
        .into_iter()
        .map(change_to_str)
        .map(|f| f.replace("DirEntry(", ""))
        .collect();
    // println!("{:#?}", new_f);
    check_files(args, new_f);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read ");

    if input.as_str().trim().len() > 1 {
        println!("Please type y or n");
    }

    input
}

#[allow(clippy::single_char_pattern)]
fn search(f: String, file_name: &str, found_file: &mut bool) -> bool {
    let temp = f.split("/").collect::<Vec<&str>>();
    let length = temp.len();
    if temp[length - 1] == file_name {
        *found_file = true;
        println!("file found at {}", f);
        println!("Do you want to open it in VS code? (y/n)");

        let typed = get_input();
        // println!("{:?}", typed.len());

        // println!("{:?}", typed);
        match typed.as_str() {
            "y\n" => {
                println!("Opening file");
                Command::new(VS_CODE)
                    .arg(f)
                    .spawn()
                    .expect("Failed to launch software.");

                // uncomment this to open it in default file explorer
                // open_file(f.as_str())
            }
            "n\n" => {
                println!("Ok, Goodbye");
            }

            &_ => {}
        }
        return true;
    }
    false
}

#[allow(unused)]
fn open_file(path: &str) {
    match open::that(Path::new(path)) {
        Ok(_) => println!("Opened"),
        Err(e) => println!("Error: {}", e),
    }
}

fn change_to_str(f: DirEntry) -> String {
    format!("{:?}", f)
}

fn return_splitted_path(f: String) -> String {
    let (first, _) = f.split_at(1).1.split_at(f.len() - 3);
    String::from(first)
}
