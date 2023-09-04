# Dark and Darker Game Updater

This is a simple CLI (Command-Line Interface) updater for the Dark and Darker
game. It downloads or verifies the latest files of the game installed.

## Prerequisites

Before running the updater, ensure that you have the following dependencies
installed:

## Installation

1. Clone the repository or download the source code files.
2. Install Rust and the required dependencies as mentioned in the prerequisites.
3. Open the terminal or command prompt and navigate to the project directory.
4. Build and install the application using the following command:

```
cargo build --release
cargo install dad-updater --path .
```

## Usage

After building and installing the application, The updater can be run using the
following command:

```
DAD_PATH="/home/thoxy/.local/share/bottles/bottles/Dark-and-Darker/drive_c/Program Files/IRONMACE/Dark and Darker/" dad-updater
```

### Optimized Use with [Bottles](https://bottles.io/)

1. Create bottles for gaming.
2. Add the following required dependencies: allfonts, vcredist2019, dotnet48.
3. Install the
   [Blacksmith Launcher](https://webdown.darkanddarker.com/Blacksmith%20Installer.exe)
   in the bottles.
4. Launch the **Blacksmith Launcher** and log in to your account.
5. Click the play/install button on the launcher.
6. Close the launcher.
7. Click on the three dots, browse the file, go one directory up, and locate the
   **Dark and Darker** directory inside the **IRONMACE** directory. Copy the
   path of this directory to the environment variable **DAD_PATH** in the
   bottles' environment settings.
8. Modify the launch command to `dad-updater %command%`.

To further optimize Bottles, you can:

- Activate Feral Gamemode.
- Set up the Steam environment.
- Use Proton-GE as the executor.

If you want to use Proton-GE, you need to set the following environment
variables:

- STEAM_COMPAT_CLIENT_INSTALL_PATH=$HOME/.steam/steam
- STEAM_COMPAT_DATA_PATH=$HOME/.local/share/Steam/steamapps/compatdata

For optimal performance on an AMD card, add the following two environment
variables:

- RADV_PERFTEST=gpl,sam,nggc
- VK_ICDR_PATH=/usr/share/vulkan/icd.d/radeon_icd.x86_64.json

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
