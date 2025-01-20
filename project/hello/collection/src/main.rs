use std::collections::HashMap;

fn main() {
    println!("Welcome to the vector collection");
    // Below is Creating new empty vector  hold value of i32 type
    let mut v: Vec<i32> = Vec::new();
    //Below vec is a macro. Actually Vec<T> This is generic type we can
    // store any type of data. In below case RUST will take according to the data 
    // U ahve provided in [] this. vec! is a macro
    let v1 = vec![1, 2, 3, 4];
    //let data = &v1[100]; // panic becz 100 index is not present
    // In above case we have provided i32 value so RUST can infer the type
    // v is Vec<i32> and type annotation not required
    v.push(10);
    v.push(20);
    // Above is pushing element to the vector. vector will free when it goes
   // out of scope
   // reading vector element
    let s = v.get(1); // This one return option<T>
    let second: &i32 = &v1[1];
    println!("The second element v1 = {}", second);
    println!("The second element v = {:?}", s);
    // If index is out of range it match will go to None
    match v1.get(2) {
        Some(second) => println!("Inside match the second element = {}", second),
        None => println!("There is no second element"),
    }
    // Print vector using for loop. Here we have use borrow for &v1 becz if we
    // not use borrow we can not use v1 from next time becz it moves to i
    for i in &v1 {
	println!("in loop V1 {}", i);
    }
    // Creating New String
    let data = "Satya Weds Liza";
    let s = data.to_string();
    println!("Satring = {}", s);
    let mut s1 = "Tinku Weds Liza".to_string();
    println!("Satring = {}", s1);
    // Appending data to String
    s1.push_str(" is Only for Tinku");
    println!("After Appending = {}", s1);
    // String concatination
    //let s3 = s + s1; // Error becz we have to pass &s1 because add function expect parameter
                     // & fn add(self, s: &str) -> String { this function ll call when we use + 
    let s3 = s + &s1;
    println!("After Concatination s3 = {}", s3);
    // Hash Map : Stores data in Heap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 50);
    //scores.insert(String::from("Red"), 40);
    //getting data from HashMap
    let TeamName = String::from("Green");
    let scor = scores.get(&TeamName);
    match scor {
	Some(scor) => println!("Team value = {}",scor),
	None => println!("Invalid Team Name"),
    }
    for (key, value) in &scores {
	println!("Key = {} and Value = {}", key, value);
    }
    // Adding new value if there is no value is present
    scores.entry(String::from("Red")).or_insert(500);
    println!("{:?}", scores);
}
