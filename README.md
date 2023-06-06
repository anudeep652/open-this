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

```
cargo run FILE_OR_FOLDER_NAME [options]


OPTIONS:
    code    Open the file/folder in vs code
```

> Remember to replace FILE_OR_FOLDER_NAME with the file or folder you want to open and if we dont specify the options, it will open the file/folder in default software launcher.

<br>
OR
<br>
<br>
Create an executable to run it locally anywhere in your local machine after cloning the repo

```sh
cargo install --path .
```

Now you can run anywhere in your terminal like this:

```
open-this FILE_OR_FOLDER_NAME [OPTIONS]

OPTIONS:
    code    Open the file/folder in vs code
```

<br>

## Todo

- [ ] Add support for opening the file/folder in desired editor
- [ ] Make codebase more readable
- [ ] Add better error handling
- [x] <strike> get username of OS </strike>
- [ ] Multiple OS support
- [ ] Search indication

<br>

## Contributing

If you are a rustacean and willing to contribute, feel free to open a PR.
