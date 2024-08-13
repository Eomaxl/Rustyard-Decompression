# Rust-Decompression

RustDecompress is a command-line utility written in Rust that allows you to efficiently extract files from a ZIP archive. It’s designed to handle various file types within the archive, including directories, files with comments, and more. The tool is modularized for easy maintenance and extension.

# Features
Command-line Arguments: Easily specify the ZIP file to extract.<br>
File Extraction: Extracts all files and directories from the ZIP archive, preserving their structure.<br>
File Comments: Displays any comments associated with the files within the archive.<br>
Unix Permissions: Preserves file permissions when extracting on Unix systems.<br>

# Prerequisites
Before you can run RustDecompress, you need to have the following installed:<br>

Rust (latest stable version): Install it from Rust's official website.<br>
Installation<br>
Clone the Repository<br>

bash<br>
Copy code<br>
git clone https://github.com/yourusername/RustDecompress.git<br>
cd RustDecompress<br>
Build the Project<br>

Navigate to the project directory and build the project using Cargo, Rust’s package manager and build system:<br>

bash<br>
Copy code<br>
cargo build --release<br>
Run the Application <br>
cargo run <zip_filename>

# Once the build is complete, you can run the application with:

bash<br>
Copy code<br>
cargo run --release -- <filename><br>
Replace <filename> with the path to your ZIP file.<br> 

# Usage<br>
After installation, you can use RustDecompress from the command line as follows:<br>

bash<br>
Copy code<br>
./RustDecompress <filename><br>
<filename>: The path to the ZIP file you want to extract.<br>
Example<br>
Here’s an example of how to use RustDecompress:<br>

bash<br>
Copy code<br>
./RustDecompress example.zip<br>
This command will extract all files and directories from example.zip to the current directory.<br>

# Project Structure
src/ <br>
main.rs: Entry point of the program.<br>
args.rs: Handles command-line argument parsing.<br>
extractor.rs: Contains the logic for extracting files from the ZIP archive and preserving Unix file permissions.<br>

# Contributing
Contributions are welcome! If you have any ideas, feel free to fork the repository and submit a pull request.

# Fork the project.
Create a new branch (git checkout -b feature-branch).<br>
Commit your changes (git commit -m 'Add some feature').<br>
Push to the branch (git push origin feature-branch).<br>
Open a pull request.

# Credits
This project was inspired by a tutorial by Akhil Sharma on YouTube. You can watch the original video here.
