struct Student {
    id: i32,
    name: String,
    active: bool
}

struct StudentNotStruct(i32, String, bool);

fn main() {
    let mut student_one: Student = Student {
        id: 1000,
        name: "Gabriel".to_string(),
        active: true
    };
    println!("{:?}", student_one.name);

    student_one.active = false;

    let student_two: Student = create_student(1001, "Daniel".to_string());
    println!("{:?}", student_two.name);

    let student_one_clone: Student = Student{ ..student_one };
    println!("{:?}", student_one_clone.name);

    let student_three: StudentNotStruct = StudentNotStruct(1002, "Nicolas".to_string(), true);
    println!("{:?}", student_three.1);
}   

fn create_student(id: i32, name: String) -> Student{
    Student {
        id: id,
        name: name,
        active: true
    }
}
