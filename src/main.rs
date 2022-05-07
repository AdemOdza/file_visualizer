use std::{env, fs};
use std::fs::metadata;
use std::path::Path;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    for filename in args {
        println!("Filename: {}", filename);
        let path_str = format!("./{}", filename);
        let curr_path = Path::new(&path_str);
        display_tree(curr_path, 0);
        println!("");
        
    }

    //Iterate through arguments
    //For each argument/file path
    // // Print name of directory
    // // Print tree of directory
}


fn display_tree(filepath: &Path, depth: i32){
    let file_info = metadata(filepath).unwrap();

    if file_info.is_file(){
        let name = filepath.file_name().expect("Error reading OsStr filename");
        println!("{}", name.to_str().expect("Error reading filename"));
        todo!("Add");

    }
    else {

    }
    //If file is last, print L
    // else, print |-
    //IF its a file, print - Name

}