mod cli;

use std::env;
use std::fs;
use std::io;
use std::fs::DirEntry;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    cli::display_cli();


    let directory = get_path(); 
    let path = Path::new(&directory);

    if path.is_dir() {
        env::set_current_dir(path).expect("Failed to change directory");
        

        // List files in the directory
        match fs::read_dir(".") {
            Ok(entries) => {   
                match is_directory_empty(&directory) {
                    Ok(is_empty) => {
                        if is_empty {
                            cli::error();
                        } else {
                            create_nested_directories(&directory);

                            for entry in entries {                                                                                         
                                let entry = entry.expect("Failed to read entry");                                                          
                                match get_extension(&entry).as_str() {                                                                     
                                    ref ext if ["jpg","jpeg","png"].contains(&ext) => move_file_to_subdir(&entry, "images"),               
                                    ref ext if ["mp3", "wav", "flac", "aac"].contains(&ext) => move_file_to_subdir(&entry, "audio_files"), 
                                    ref ext if ["py", "js", "rs", "java"].contains(&ext) => move_file_to_subdir(&entry, "code_files"),     
                                    ref ext if ["pdf", "epub", "docx"].contains(&ext) => move_file_to_subdir(&entry, "pdf_files"),         
                                    _ => Ok({}) // Handle other cases if necessary                                                         
                                }.expect("Reason");                                                                                        
                            }                                                                                                              
                            cli::done(); 

                            let txt_file_path = "READ_ME.txt";
                            let mut file = File::create(txt_file_path).expect("Failed to create the txt file.");

                            let content = "
-----------------------------------------------------------
          File Management Tool - Completed Log
-----------------------------------------------------------

Thank you for using the File Management Tool!

This tool has been designed to help you organize and manage 
your files efficiently by sorting them into appropriate 
directories.

You can view the source code and contribute to the development 
of this tool on GitHub:

GitHub Account: zpeteman
GitHub Link: https://github.com/zpeteman

-----------------------------------------------------------
Tool Summary:
-----------------------------------------------------------
- Files have been successfully organized into respective 
directories.
- Image files have been moved to the 'images' directory.
- Audio files have been moved to the 'audio_files' directory.
- Code files have been moved to the 'code_files' directory.
- PDF and document files have been moved to the 'pdf_files' 
directory.

-----------------------------------------------------------
Thank you again for using the tool. Happy organizing!

If you have any feedback or questions, feel free to reach out 
to me on GitHub!

-----------------------------------------------------------
                                        ";
                            file.write_all(content.as_bytes())
                                .expect("Failed to write to txt file.");

                        }
                    }
                    Err(err) => {
                        println!("Error when checking if directory is empty: {}", err);
                    }
                }
            }
            Err(e) => eprintln!("Failed to read directory contents: {}", e),
        }
    } else {
        eprintln!("Invalid directory path: {}", directory);
    }
}

fn get_path() -> String {
    loop {
        let mut el_path = String::new();
        let mut path = String::from("/home/");
        let mut username = whoami::username();
        path.push_str(&mut username);
        path.push_str("/");
        println!("enter the path of ur directory: ");

        io::stdin()
            .read_line(&mut el_path) .expect("Failed to get value.");
        el_path = el_path.trim().to_string();
        path.push_str(&el_path);
        
        if path.chars().last() != Some('/') {
           path.push_str("/") 
        }

        return path;
    }
}

fn create_nested_directories(path: &String) {
    let audio_files = format!("{}audio_files", path);  // Create the full path for each directory
    let images = format!("{}images", path);
    let pdf_files = format!("{}pdf_files", path);
    let code_files = format!("{}code_files", path);

    // Create the directories
    fs::create_dir_all(audio_files).ok();   // Ignore the result if you don't care about errors
    fs::create_dir_all(images).ok();
    fs::create_dir_all(pdf_files).ok();
    fs::create_dir_all(code_files).ok();
}

fn get_extension(file: &DirEntry) -> String {
    let path = file.path(); // Get the path from DirEntry
    
    // Check if the file has an extension
    let x = match path.extension() {
        Some(ext) => ext.to_string_lossy().to_string(), // If extension exists, return it as a String
        None => "this is a dir.".to_string(), // If no extension, return a default message
    };
    return x;
}

fn move_file_to_subdir(file_path: &DirEntry, subdir_path: &str) -> std::io::Result<()> {
    let file = file_path.path();
    let subdir = Path::new(&subdir_path);

    // Construct the new path for the file
    let new_file_path = subdir.join(file.file_name().unwrap()); // Keep the same file name

    // Move the file into the subdirectory
    fs::rename(file, new_file_path)?;
    Ok(())
}

fn is_directory_empty(directory: &String) -> std::io::Result<bool> {
    let mut entries = fs::read_dir(directory)?;
    let first_entry = entries.next();
    Ok(first_entry.is_none())
}
