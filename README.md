# Rust Static File Server

This project is a simple Rust-based static file server using the Actix Web framework. It serves MP3 files from a specified directory structure and supports dynamic URL paths for accessing files.

## Features

- Serves MP3 files from a static directory.
- Supports dynamic URL paths for accessing files.
- No user upload functionality for enhanced security.

## Directory Structure

Ensure your directory structure follows this format:

```
.
├── static
│   ├── oald
│   │   ├── us
│   │   │   ├── level1
│   │   │   │   ├── level2
│   │   │   │   │   └── file1.mp3
│   │   │   └── file1.mp3  # For legacy support
│   ├── xx
│   │   ├── us
│   │   │   ├── level1
│   │   │   │   ├── level2
│   │   │   │   │   └── file2.mp3
│   │   │   └── file2.mp3  # For legacy support
│   ├── yy
│   │   ├── ams
│   │   │   ├── level1
│   │   │   │   ├── level2
│   │   │   │   │   └── file3.mp3
│   │   │   └── file3.mp3  # For legacy support
```

## Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/your_username/rust-static-file-server.git
   cd rust-static-file-server
   ```

2. Ensure you have Rust installed. If not, install Rust using [rustup](https://rustup.rs/):

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. Build the project in release mode:

   ```sh
   cargo build --release
   ```

## Usage

1. Start the server:

   ```sh
   cargo run
   ```

2. Access MP3 files via your browser or any HTTP client:

   - Legacy structure:
     - `http://127.0.0.1:8000/oald/us/file1.mp3`
     - `http://127.0.0.1:8000/xx/us/file2.mp3`
     - `http://127.0.0.1:8000/yy/ams/file3.mp3`
   - New structure:
     - `http://127.0.0.1:8000/oald/us/level1/level2/file1.mp3`
     - `http://127.0.0.1:8000/xx/us/level1/level2/file2.mp3`
     - `http://127.0.0.1:8000/yy/ams/level1/level2/file3.mp3`

## Autostart on macOS using `launchd`

To configure the server to start automatically on macOS, create a `launchd` plist file.

1. Create the `launchd` plist file at `~/Library/LaunchAgents/com.rust.dict_server.plist` with the following content:

   ```xml
   <?xml version="1.0" encoding="UTF-8"?>
   <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
   <plist version="1.0">
   <dict>
       <key>Label</key>
       <string>com.rust.dict_server</string>
       <key>ProgramArguments</key>
       <array>
           <string>/Users/yourusername/Library/DictServer/target/release/dict_server</string>
       </array>
       <key>WorkingDirectory</key>
       <string>/Users/yourusername/Library/DictServer</string>
       <key>RunAtLoad</key>
       <true/>
       <key>KeepAlive</key>
       <true/>
   </dict>
   </plist>
   ```

2. Replace `/Users/yourusername/Library/DictServer/target/release/dict_server` with the actual path to your compiled executable.

3. Load the plist file to `launchd`:

   ```sh
   launchctl load ~/Library/LaunchAgents/com.rust.dict_server.plist
   ```

4. Verify that the service is running:

   ```sh
   launchctl list | grep com.rust.dict_server
   ```

5. To unload the service:

   ```sh
   launchctl unload ~/Library/LaunchAgents/com.rust.dict_server.plist
   ```

## Customization

To customize the directory structure or change the file serving logic, modify the `main.rs` file.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.