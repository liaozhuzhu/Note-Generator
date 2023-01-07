use std::fs::File;
use std::fs::create_dir_all;
use std::env;
use std::fs::write;
use std::fs::read_to_string;
use std::collections::HashMap; // [Semester: [courses]]

fn main() {
    // Create directory "Courses" in external directory
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("Courses");
    create_dir_all(&current_dir).unwrap();

    let mut map: HashMap<String, Vec<&str>> = HashMap::new();
    map.insert(String::from("Spring 2023"), vec!["Algo", "Analysis", "Gen Ed"]);
    let mut v: Vec<i32> = Vec::new();
    let mut test_vector: Vec<&str> = vec!["Hello", "This", "Is", "a", "test"];
    for course in  test_vector{
        let file_name = format!("./Courses/{}.md", course);
        let mut file = File::create(&file_name)
            .expect("Error creating file...");   
        // file.write_all(b"# Test")
        //     .expect("Error writing to file..."); 
        write(format!("./Courses/{}.md", course), format!("# {}", course));
    }
    // for (key, value) in map {
    //     println!("Semester: {}", key);
    //     println!("Courses: {:?}", value);
    // }
}
