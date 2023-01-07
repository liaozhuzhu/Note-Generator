use std::fs::File;
use std::fs::create_dir_all;
use std::env;
use std::fs::write;
use std::collections::HashMap; // [Semester: [courses]]

fn main() {
    let mut current_dir = env::current_dir().unwrap();


    let mut schedule: HashMap<String, Vec<&str>> = HashMap::new();
    schedule.insert(String::from("Spring 2023"), vec!["Algo", "Analysis", "Gen Ed"]);

    // Create semester dir inside Courses
    for (semester, _classes) in &schedule {
        current_dir.push(format!("./{}", semester));
        create_dir_all(&current_dir).unwrap();
    }

    for (semester, classes) in &schedule {
        for class in classes {
            let file_name = format!("./{}/{}.md", semester, class);
            File::create(&file_name).expect("Error creating file...");   

            // Write title on each md file
            write(format!("./{}/{}.md", semester, class), format!("# {}", class));
        }
    }

    // if let Some(courses) = schedule.get("Spring 2023"){
    //     for course in courses {
    //         // Create md files of each course
    //         let file_name = format!("./Courses/{}.md", course);
    //         File::create(&file_name).expect("Error creating file...");   

    //         // Write title on each md file
    //         write(format!("./Courses/{}.md", course), format!("# {}", course));
    //     }
    // }
}
