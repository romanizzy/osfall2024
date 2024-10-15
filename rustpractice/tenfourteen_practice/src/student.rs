//  define a struct student
//  with name and major
#[derive(Debug)]

pub struct Student {
    name: String,
    major: String,
}

//  provide 3 methods
//  create a new student
//  change name
//  change major
impl Student {
    pub fn new_student(n: String, m: String)-> Self{
        Student{
            name: n,
            major: m
        }
    }
    pub fn introduce_yourself(&self){
        println!("My name is {} and my major is {}", self.name, self.major)
    }
    pub fn change_name(&mut self, new_name: String){
        self.name = new_name;
    }
    pub fn change_major(&mut self, new_major: String){
        self.major = new_major;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_creation(){
        let s = Student::new_student("Alex".to_string(), "Computer Science".to_string());
        assert_eq!(s.name, "Alex");
    }

    #[test]
    fn test_major_change(){
        let mut s = Student::new_student("Alex".to_string(), "Computer Science".to_string());
        assert_eq!(s.major, "Computer Science");
        s.change_major("Biology".to_string());
        assert_eq!(s.major, "Biology");
    }
}