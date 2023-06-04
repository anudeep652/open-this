use std::{
    fs::{self, DirEntry},
    io::{self},
    path::{Path, PathBuf},
    process::{self, Command},
};

pub const USER: &str = "anudeep";
pub static mut HOME_DIR: String = String::new();
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
fn check_files(file_args: &Vec<String>, searched: &mut [String]) {
    let mut found_file = false;
    searched
        .iter_mut()
        .map(|f| return_splitted_path(f.to_owned()))
        .any(|f| search(f, file_args[1].as_str(), &mut found_file));

    searched
        .iter()
        .map(|f| return_splitted_path(f.to_owned()))
        .for_each(|f| {
            if is_dir(f.as_str()) {
                let paths = read_dirs(f.as_str());

                search_all_dirs(paths, file_args)
            }
        })
}

pub fn search_all_dirs(files: Vec<DirEntry>, args: &Vec<String>) {
    let mut new_f: Vec<_> = files
        .into_iter()
        .map(change_to_str)
        .map(|f| f.replace("DirEntry(", ""))
        .collect();
    // println!("{:#?}", new_f);
    check_files(args, &mut new_f);
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
                process::exit(1)

                // uncomment this to open it in default file explorer
                // open_file(f.as_str())
            }
            "n\n" => {
                println!("Ok, Goodbye");
                process::exit(1)
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

pub fn set_user(user: &str) {
    unsafe { HOME_DIR = ["/home/", user].concat() }
}
