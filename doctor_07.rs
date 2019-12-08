use std::io;
//needed for user input

//Structure:
struct Subject{
	class_section: String,
	course_code: String,
}

struct Student{
	name: String,
	s_num: String,
	subjects: Vec<Subject>
}

struct Students{
	students: Vec<Student>
}

fn add_a_student(nameVar: String, s_numVar: String) -> Student{
	let mut NewStudent = Student{
		name: nameVar,
		s_num: s_numVar,
		subjects: Vec::new()
	};
	return NewStudent
}

fn add_a_subject(class_sectionVar: String, course_codeVar: String) -> Subject{
	let mut NewSubject = Subject{
		class_section: class_sectionVar,
		course_code: course_codeVar
	};
	return NewSubject
}

fn check_if_s_num_exists(s_numVar: &String, Students_list: &Students) -> bool{
	if Students_list.students.len() == 0{
		//snum doesnt exist if there are no students yet
		return false
	}else{
		let mut count = 0;
		while count < Students_list.students.len(){
			//if an snum is the same in any of our list, it returns true
			if s_numVar == &Students_list.students[count].s_num{
				return true
			}else{
				count = count + 1;
			}
		}
			//it has already gone through all the possible studnets
			return false
	}
}

fn main(){
	let mut Students_list = Students{
		students: Vec::new()
		//creates an empty students vector
		//we just push whatever student
	};

	loop{
		println!("[1] Add Student");
		println!("[2] View ALL Students");
		println!("[3] Add subject to student");
		println!("[4] Delete Student");
		println!("[5] Exit");
		println!("Enter choice: ");

		//initialize students in Students_list structure to empty

		let mut choice = String::new();
		io::stdin().read_line(&mut choice);
		let choice:i32 = choice.trim().parse().expect("Error");

		if choice == 1 {
			//we use add_a_student function

			//ask for name
			println!("Enter name: ");
			let mut nameVar = String::new();
			io::stdin().read_line(&mut nameVar).expect("Error");

			//ask for studentNum
			println!("Enter student number: ");
			let mut s_numVar = String::new();
			io::stdin().read_line(&mut s_numVar).expect("Error");
			
			//we must only create a student, if the snum does not exist
			if !(check_if_s_num_exists(&s_numVar, &Students_list)){
				//pass these variables and push them into the students vector
				Students_list.students.push(add_a_student(nameVar, s_numVar));
				println!("{}", Students_list.students.len());
			}else{
				println!("Student number already exists!")
			}
		}else if choice == 2{
			if Students_list.students.len() == 0{
				println!("There are no students in the record")
			}else{
				let mut count = 0;
				while count < Students_list.students.len(){
				//this gets the length of students in Students_list
					println!("Entry#{}", count+1);
					println!("Name: {}", Students_list.students[count].name);
					println!("Student Number: {}", Students_list.students[count].s_num);
					if Students_list.students[count].subjects.is_empty(){
						println!("{} has no subjects", Students_list.students[count].name);
					}else{
						let mut countSubj = 0;
						while countSubj < Students_list.students[count].subjects.len(){
							println!("Subject#{}", countSubj+1);
							println!("Class Section: {}", Students_list.students[count].subjects[countSubj].class_section);
							println!("Course Code: {}", Students_list.students[count].subjects[countSubj].course_code);
							countSubj = countSubj + 1;
						}
					}
					count = count + 1;
				}
			}

		}else if choice == 3{
			println!("Enter student number: ");
			let mut s_numVar = String::new();
			io::stdin().read_line(&mut s_numVar).expect("Error");
			
			//we can only add the student if his snum is already in the record
			if check_if_s_num_exists(&s_numVar, &Students_list){
				//we know this student exists
				//we need to go through our list to see which student
				let mut count = 0;
				while count < Students_list.students.len(){
					if s_numVar == Students_list.students[count].s_num{
						//ask for class section
						println!("Enter class section: ");
						let mut class_sectionVar = String::new();
						io::stdin().read_line(&mut class_sectionVar).expect("Error");

						//ask for course code
						println!("Enter course code: ");
						let mut course_codeVar = String::new();
						io::stdin().read_line(&mut course_codeVar).expect("Error");

						Students_list.students[count].subjects.push(add_a_subject(class_sectionVar, course_codeVar));
						println!("Course has been added!");
						break;
					}else{
						count = count + 1;
					}
				}
			}else{
				println!("Student does not exist!")
			}
		}else if choice == 4{
			println!("Enter student number: ");
			let mut s_numVar = String::new();
			io::stdin().read_line(&mut s_numVar).expect("Error");
			
			//we can only add the student if his snum is already in the record
			if check_if_s_num_exists(&s_numVar, &Students_list){
				//we know this student exists
				//we need to go through our list to see which student
				let mut count = 0;
				while count < Students_list.students.len(){
					if s_numVar == Students_list.students[count].s_num{
						Students_list.students.remove(count);
						println!("Student has been deleted!");
						break;
					}else{
						count = count + 1;
					}
				}
			}else{
				println!("Student does not exist!")
			}
		}else if choice == 5{
			break;		//leaves the loop body
		}else{
			println!("Invalid choice.");
		}
	}

}