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
    // Print vector using for loop
    for i in &v1 {
	println!("in loop V1 {}", i);
    }
}
