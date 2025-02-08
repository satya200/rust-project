// Below is example for if, for and while example
fn if_control() {
    let a = 1;
    let cond = true;
    if a > 9 {
        println!("High Temp");
    } else if a > 5 {
        println!("Mid Temp");
    } else {
        println!("Low Temp");
    }

    let wether = if a > 9 {"sony"} else {"no sony"};
    println!("wether is {}", wether);

    println!("It is {}", if a > 9 {"Hi"} else {"hello"});
    let number = if cond { 5 } else { 9 };
    //let number = if cond { 5 } else { "satya" }; //error becz number type should define compile time and here the type is define 2 differently which cause error
    println!("number is {}", number);
}

fn while_for_loop() {
    let mut a = 1;
    loop {
        a = a + 1;
        println!("Welcome Satya");
        if a == 5 {
            println!("breaking loop");
	    break;
        }
    }
    while a != 10 {
        println!("a = {}", a);
        a = a + 1;
    }
}

fn print_array() {
    let a = [1, 2, 3, 4, 5];
    let mut index = 0;
    while index < 5 {
        println!("a = {}", a[index]);
        index = index + 1;
    }
    for element in a.iter() {
        println!("The value is {}", element);
    }
    for x in 1..11 {
        println!("x = {}", x);
    }
    for y in (1..6).rev() {
        println!("y = {}", y);
    }
}

fn fibonaci_series(num: i32) {
    let mut index: i32 = 0;
    let mut fibo_num: i32 = 0;
    println!("Going to print fibonaci series of {} numbers", num);
    while index < num {
	fibo_num = fibo_num + index;
	println!("fibonaci series of {} numbers", fibo_num);
        index = index + 1;
    }
}

fn my_match() {
    let code = 30;
    // match statement is very power ful tool in RUST. It covers all condition and check at compile time
    let country = match code {
        30 => "UK",
        40 => "USA",
        75 => "ODISHA",
        1..=1000 => "UNKNOWN",
        _ => "INVALID"
    };
    println!("code = {}", country);
}

fn main() {
    if_control();
    while_for_loop();
    print_array();
    my_match();
    fibonaci_series(5);
}
