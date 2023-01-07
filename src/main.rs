use std::fs::File;
use std::fs::create_dir_all;
use std::env;
use std::fs::write;
use std::collections::HashMap; // [Semester: [courses]]

fn main() {
    // Create directory "Courses" in external directory
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("Courses");
    create_dir_all(&current_dir).unwrap();

    let mut map: HashMap<String, Vec<&str>> = HashMap::new();
    map.insert(String::from("Spring 2023"), vec!["Algo", "Analysis", "Gen Ed"]);
    if let Some(courses) = map.get("Spring 2023"){
        for course in courses {
            // Create md files of each course
            let file_name = format!("./Courses/{}.md", course);
            File::create(&file_name).expect("Error creating file...");   

            // Write title on each md file
            write(format!("./Courses/{}.md", course), format!("# {}", course));
        }
    }
    // for (key) in map.get("Spring 2023") {
    //     for name in key {
    //         println!("{}", name);
    //     }
    // }
}
