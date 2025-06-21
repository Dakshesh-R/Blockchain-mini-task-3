use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Clone)]
struct Student {
    maths: f64,
    physics: f64,
    chemistry: f64,
    biology: f64,
    english: f64,
    total: f64,
    average: f64,
    grade: String,
}

fn main() {
    let mut students: HashMap<String, Student> = HashMap::new();

    loop {
        println!("\n--- Student Report Card ---");
        println!("1. Add Student");
        println!("2. View Student");
        println!("3. Update Student");
        println!("4. Delete Student");
        println!("5. List All Students");
        println!("6. Exit");

        print!("Choose option: ");
        io::stdout().flush().unwrap();
        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();

        match option.trim() {
            "1" => {
                let name = get_input("Enter student name:");
                if students.contains_key(&name) {
                    println!("Student already exists.");
                    continue;
                }
                let student = input_student_marks();
                students.insert(name.clone(), student);
                println!("Student '{}' added.", name);
            }
            "2" => {
                let name = get_input("Enter student name:");
                if let Some(s) = students.get(&name) {
                    display_student(&name, s);
                } else {
                    println!("Student not found.");
                }
            }
            "3" => {
                let name = get_input("Enter student name to update:");
                if students.contains_key(&name) {
                    let student = input_student_marks();
                    students.insert(name.clone(), student);
                    println!("Student '{}' updated.", name);
                } else {
                    println!("Student not found.");
                }
            }
            "4" => {
                let name = get_input("Enter student name to delete:");
                if students.remove(&name).is_some() {
                    println!("Student '{}' deleted.", name);
                } else {
                    println!("Student not found.");
                }
            }
            "5" => {
                if students.is_empty() {
                    println!("No students available.");
                } else {
                    println!("Students:");
                    for name in students.keys() {
                        println!("- {}", name);
                    }
                }
            }
            "6" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option."),
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn input_student_marks() -> Student {
    let maths = get_mark("Maths");
    let physics = get_mark("Physics");
    let chemistry = get_mark("Chemistry");
    let biology = get_mark("Biology");
    let english = get_mark("English");

    let total = maths + physics + chemistry + biology + english;
    let average = total / 5.0;
    let grade = match average {
        90.0..=100.0 => "A",
        75.0..=89.9 => "B",
        60.0..=74.9 => "C",
        _ => "D",
    }
    .to_string();

    Student {
        maths,
        physics,
        chemistry,
        biology,
        english,
        total,
        average,
        grade,
    }
}

fn get_mark(subject: &str) -> f64 {
    loop {
        let input = get_input(&format!("Enter marks for {}: ", subject));
        match input.parse::<f64>() {
            Ok(m) => return m,
            Err(_) => println!("Invalid number. Try again."),
        }
    }
}

fn display_student(name: &str, s: &Student) {
    println!("\n--- Report Card: {} ---", name);
    println!("Maths:      {}", s.maths);
    println!("Physics:    {}", s.physics);
    println!("Chemistry:  {}", s.chemistry);
    println!("Biology:    {}", s.biology);
    println!("English:    {}", s.english);
    println!("Total Marks: {}", s.total);
    println!("Average:     {:.2}", s.average);
    println!("Grade:       {}", s.grade);
}