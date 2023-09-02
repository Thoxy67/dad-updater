# Dark and Darker Game Updater

This is a simple CLI (Command-Line Interface) updater for the Dark and Darker
game. It downloads or verifies the latest files of the game installed.

## Prerequisites

Before running the updater, ensure that you have the following dependencies
installed:

## Usage

To use the Dark and Darker game updater, follow these steps:

1. Clone the repository or download the source code files.
2. Install Rust and the required dependencies as mentioned in the prerequisites.
3. Open the terminal or command prompt and navigate to the project directory.
4. Build and run the application using the following command:

```
cargo run --release
```

This command will compile the code and execute the updater.

## Installation

1. Clone the repository or download the source code files.
2. Install Rust and the required dependencies as mentioned in the prerequisites.
3. Open the terminal or command prompt and navigate to the project directory.
4. Build and install the application using the following command:

```
cargo build --release
cargo install dad-updater --path .
```

## Command-Line Arguments

The updater accepts the following command-line arguments:

- `-p`, `--path`: Specify the path to the game installation directory. This
  argument is optional and defaults to the value of the `DAD_PATH` environment
  variable.
- `-s`, `--speed`: Set the download speed limit in bytes per second. This
  argument is optional and defaults to the value of the `DAD_DOWNLOAD_SPEED`
  environment variable or `0` if not provided (`0` = `no limit`).
- `-t`, `--threads`: Set the number of simultaneous download threads. This
  argument is optional and defaults to the value of the `DAD_THREADS`
  environment variable or `5` if not provided.

## Functionality

The updater performs the following tasks:

1. Reads the file URLs and other information from the Dark and Darker
   PatchFileList.txt hosted on `http://cdn.darkanddarker.com`.
2. Downloads each file using multiple threads with a progress bar indicating the
   download status.
3. Implements a download speed limit if provided to regulate the download rate.
4. Verifies the integrity of downloaded files using SHA256 and compares the file
   size to ensure they are up to date.

## License

This project is licensed under the [MIT License](LICENSE).

## Contributions

Contributions to this project are welcome. If you find any issues or want to
enhance the functionality, feel free to open a pull request.
