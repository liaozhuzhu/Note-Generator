use std::fs::File;
use std::fs::create_dir_all;
use std::env;
use std::fs::write;
use std::collections::HashMap; // [Semester: [courses]]
use std::io;

fn main() {

    // let mut input_map = HashMap::new();
    println!("Enter 'Semester: Course1, Course2, Course3'");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Split input into vector key-value pairs
    let pairs: Vec<_> = input
        .trim()
        .split(": ")
        .map(|s| s.split(", ").collect::<Vec<_>>())
        .collect();

    let mut classes = Vec::new();
    for class in &pairs[1] {
        classes.push(*class);
    }

    let mut schedule: HashMap<String, Vec<&str>> = HashMap::new();
    schedule.insert(format!("{}", pairs[0][0]), classes);

    // Create semester dir inside Courses
    for (semester, _classes) in &schedule {
        let mut current_dir = env::current_dir().unwrap();
        current_dir.push(format!("../{}", semester));
        create_dir_all(&current_dir).unwrap();
    }

    // Add course md files to semester dir
    for (semester, classes) in &schedule {
        for class in classes {
            let file_name = format!("../{}/{}.md", semester, class);
            File::create(&file_name).expect("Error creating file...");   

            // Write title on each md file
            write(format!("../{}/{}.md", semester, class), format!("# {}", class));
        }
    }
}