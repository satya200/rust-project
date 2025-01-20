// Below program is example for ownership
fn main() {
    let s = "Satya"; // This is store in stack and the memory is present in scope. If scope complete memory also droped from stack
    println!("string: {}", s);
    {
    	let mut str = String::from("Sundar");// This memory allocate in heap
    	println!("Dynamic heap: {}", str);
    	str.push_str(" weds Liza");
    	println!("str after push = {}", str);
        let str1 = str;
    	//println!("str after push = {}", str);// error becz this ownership move concept. When str assign to str1 after that str memory got released
        let s1 = String::from("Hello");
        let s2 = s1.clone(); // This is not move this is copy concept
        println!("clone = {}, {}", s1, s2);
    }
    let str_fun = String::from("Satya Sundar Sahu");
    takes_owner(str_fun);// This is move concept
    //println!("After fun = {}", str_fun);// Error becz above is move 
    let x = 10;
    takes_owner_data(x); // This is not move this copy concept
    println!("takes_owner_data = {}", x);
    //println!("clone = {}, {}", s1, s2);
    //println!("str after push = {}", str);// Error becz memory visible to the scope only
    let str_fun1 = String::from("Satya");
    let (s3, len) = cal_length(str_fun1); // return type is tuple
    println!("return str = {} and len = {}", s3, len);
    let str_fun2 = String::from("Sundar");
    let len = cal_len_ref(&str_fun2);// This is reference method. Here str_fun2 is refernce not owning
    println!("Sundar len = {}", len);

    // Below is example for slice retun 1st word in a string
    let str_word = String::from("Satya Sundar Sahu");
    let word = string_word(&str_word);
    println!("1st word im {} = {}", str_word, word);

    let str_lit = "Hi How are you"; // This is string litreals and this is called slices also why becz it holds the reference
    println!("str_lit = {}", str_lit);
    let slice = &str_word[1..4];
    println!("slice = {}", slice);
}

fn takes_owner(str: String) {
    println!("in takes_owner = {}", str);
}

fn takes_owner_data(x: i32) {
    println!("in takes_owner_data x = {}", x);
}

fn cal_length(str: String) -> (String, usize) {
    println!("In cal_length = {}", str);
    let len = str.len();
    (str, len) // retun ownership and length
}

// Below is the reference method
fn cal_len_ref(str: &String) -> usize {
    str.len()
}
// &str is type mention for slice
fn string_word(str: &String) -> &str {
    let byte = str.as_bytes();// This ll convert string to byte array for iterate

    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
	    return &str[0..i]; // Return 0 to 1st sapce if we write str[..] means all string will consider
	}
    }
    &str[..] //return full string 0 to end
}
