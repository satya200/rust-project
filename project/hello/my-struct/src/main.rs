// This file is explain how to use struct
#[derive(Debug)]

struct Student {
    name: String,
    email: String,
    roll_no: u32,
    active: bool,
}

struct Teacher {
    name: String,
    student: Student,
}

struct Color(i32, i32, i32);

fn main() {
    let stu1 = Student {
    email: String::from("satya@gmail.com"),
    roll_no: 1,
    active: true,
    name: String::from("Satya Sundar Sahu"),
    };
    println!("stu1 data name={}\n email:{}\n roll:{}\n status:{}", stu1.name, stu1.email, stu1.roll_no, stu1.active);

    
    let mut stu2 = Student {
    email: String::from("satya@gmail.com"),
    roll_no: 1,
    active: true,
    name: String::from("Satya Sundar Sahu"),
    };
    println!("stu2 data name={}\n email:{}\n roll:{}\n status:{}", stu2.name, stu2.email, stu2.roll_no, stu2.active);
    stu2.roll_no = 2;
    stu2.email = String::from("tink@gmail.com");
    stu2.name = String::from("tinku sahu");
    println!("stu2 Update data name={}\n email:{}\n roll:{}\n status:{}", stu2.name, stu2.email, stu2.roll_no, stu2.active);

    let stu3 = build_usr(String::from("Mamuni"), String::from("mamuni@gmail.com"));
    println!("stu3 data name={}\n email:{}\n roll:{}\n status:{}", stu3.name, stu3.email, stu3.roll_no, stu3.active);

    // Creating instance from stu3 to stu4
    let stu4 = Student {
        email: String::from("rohan@gmail.com"),
        name: String::from("rohan sahu"),
        ..stu3
    };
    println!("stu4 data name={}\n email:{}\n roll:{}\n status:{}", stu4.name, stu4.email, stu4.roll_no, stu4.active);

    //  Using tuple struct
    struct Origin(u32, u32, u32);
    let col = Color(1,2,3);
    let org = Origin(1,2,3);
    println!("col = {}, {}, {}\n org = {}, {}, {}", col.0, col.1, col.2, org.0, org.1, org.2);

    println!("stu4 data using debug methode= {:?}", stu4);
    println!("stu4 data using better debug methode= {:#?}", stu4);

    // Assign student inside Teacher structure
    let teach = Teacher {
        name: String::from("Sandipan pine"),
        student: stu4
    };
    println!("Teacher data using better debug methode={}, {:#?}", teach.name, teach.student);
}

fn build_usr(email: String, name: String) -> Student {
    // In below we are changing only name and email so we wite directly name insted of name: name
    // Below methode called initshorthand insted of writing name: name we can write directly name
    Student {
        name,
        email,
        roll_no: 3,
        active: false,
    }
}
