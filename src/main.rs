use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let file_content_string = fs::read_to_string(file_name).expect("cant read file");
    let lines_in_file = file_content_string.split("\n");

    let mut interfaces = String::new();
    let mut is_active = false;

    lines_in_file.for_each(|line| {
        let mut has_pushed = false;
        if line.starts_with("export interface") {
            has_pushed = true;
            is_active = true;
            interfaces.push_str(line);
            interfaces.push_str("\n");
        }
        if is_active && line.starts_with("}") {
            is_active = false;
            interfaces.push_str(line);
            interfaces.push_str("\n");
        }
        if is_active && !has_pushed {
            interfaces.push_str(line);
            interfaces.push_str("\n");
        }
    });

    let out = "out/";
    let path = out.to_owned() + file_name;
    if Path::new(out).exists() {
        fs::remove_dir_all(out).expect("cant remove dir");
    }
    fs::create_dir(out).expect("cant create dir");
    fs::write(path, interfaces).expect("cant write file");
}
