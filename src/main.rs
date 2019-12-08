use std::io;

fn main() {
    
    // Function calling
    
    println!("Assignment # 1 - Question # 1");
    question_1();
    
    println!("Assignment # 1 - Question # 2");
    question_2();
    
    println!("Assignment # 1 - Question # 3");
    question_3();
    
    println!("Assignment # 1 - Question # 4");
    question_4();
    
    println!("Assignment # 1 - Question # 5");
    question_5();
    
}

fn question_1() {
    
    // Question # 1
    
    let marks_obtained = 95.0;
    let total_marks = 150.0;
    let percentage: f32 = (marks_obtained / total_marks) * 100.0;
    
    println!("{} marks obtained out of {}. Total percentage is {} %\n", marks_obtained, total_marks, percentage);

}

fn question_2() {
    
    // Question # 2
    
    println!("Enter any input:");
    
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("Your input is : {}\n", input);

}

fn question_3() {
    
    // Question # 3
    
    let name = "Syed Qutaiba Zamir";
    let age: i32 = 32;
    let cell_number: u32 = 03468284226;

    println!("Name is : {}", name);
    println!("Age is : {}", age);
    println!("Cell number is : {}\n", cell_number);
}

fn question_4() {
   
   // Variable declaration
   
   let a: f64 = 125.0; 
   let b: i64 = 12456;
   let ax: i64 = 1234567890;
   let s: i64 = 4043;
   let x: f64 = 2.13459;
   let dx: f64 = 1.415927;
   let c: char = 'W';
   let ux: i64 = 2541567890;

   println!("a + c is {} + {}", a, c);
   println!("x + c is {} + {}", x, c);
   println!("dx + x is {}", dx + x);
   println!("a + x is {}", a + x);
   println!("s + b is {}", s + b);
   println!("ax + b is {}", ax + b);
   println!("s + c is {} + {}", s, c);
   println!("ax + c is {} + {}", ax, c);
   println!("ax + ux is {}\n", ax + ux);

}

fn question_5() {
    
    // Question # 5
    
    let days: i64 = 1329;
    let years: i64 = days / 365;
    let weeks: i64 = (1329 - years * 365) / 7;

    println!("Years {}", years);
    println!("Weeks {}\n", weeks);

}