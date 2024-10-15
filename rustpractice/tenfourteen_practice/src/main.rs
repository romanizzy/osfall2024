mod student;
use student::Student;

fn main(){
    let mut name = "John".to_string();
    let mut major = "Computer Science".to_string();
    let mut my_student = Student::new_student(name, major);

    println!("{:?}", my_student);

    name = "Carlos".to_string();
    major = "Biology".to_string();
    my_student.change_name(name);
    my_student.change_major(major);
    my_student.introduce_yourself();
}