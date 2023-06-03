<div align="center">

# Open This

</div>

A CLI tool to search files and folder through a computer and open it (currently only in vs code)
This is a simple hobby project made by me to gain some experience in working with rust.
<br>
I felt exhaused by navigating and searching through folders and files in my computer.
<br>
<br>

## Usage

clone the repo:

```sh
git clone https://github.com/anudeep652/open-this.git
```

<br>

```sh
cd open-this
```

```sh
cargo run FILE_OR_FOLDER_NAME
```

> Remember to replace FILE_OR_FOLDER_NAME with the file or folder you want to open

<br>
OR
<br>
<br>
Create an executable to run it locally anywhere in your local machine after cloning the repo

```sh
cargo install --path .
```

Now you can run anywhere in your terminal like this:

```sh
open-this FILE_OR_FOLDER_NAME
```

> Currently it opens the file/folder only in vs code 

<br>

## Future plans

- Add support for opening the file/folder in desired editor
- Make codebase more readable 
- Add better error handling
- get username of OS
- Multiple OS support

<br>

## Contributing

If you are a rustacean and want to contribute to this project, feel free to open a PR.
