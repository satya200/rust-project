use std::mem;

fn main() {
    let  a = "satya";
    println!("a = {}", a);
    println!("a = {} and size is {}", a, mem::size_of_val(&a));
    //a = 23; // error
    /* But above imutable value a if i use againg let a = 23 it ll work */
    let a = 23;
    //let mut a = 33; // This will give warning becz already "a" become imutable
    println!("2nd time a = {} and size is {}", a, mem::size_of_val(&a));
    let a = 'x';
    println!("3rd time a = {} and size is {}", a, mem::size_of_val(&a));

    let b:i8 = 23;
    println!("1st time b = {} and size is {}", b, mem::size_of_val(&b)); // 1 byte
    let b = 234567789;
    println!("2nd time b = {} and size is {}", b, mem::size_of_val(&b)); // 4 byte

    let c:i8 = 44;
    println!("1st time c = {} and size is {}", c, mem::size_of_val(&c)); // 1 byte
    //c = 4456789; // error 
    //println!("2nd time c = {} and size is {}", c, mem::size_of_val(&c)); //error becz of over flow

    const MAX:u32 = 78; // with const key word we should not use mut and data type should mandatory
    println!("const MAX = {} and size is {}", MAX, mem::size_of_val(&MAX));
    //MAX = 21; // error
    //const MAX:u32 = 90; // reddefine error
    let r#let = 43; //Raw Identifiers  using r# we can use keywords as a varibale
    println!("let = {} and size is {}", r#let, mem::size_of_val(&r#let));

    let cub_test = 3;
    let a_cub = i32::pow(cub_test, 3); // This is the methode use for calculating cube
    println!("a_cub = {}", a_cub);

    let b_f  = 2.5;
    let b_f_cub = f64::powi(b_f, 3);
    println!("b_f = {}", b_f_cub);

    /* Below x is a tuple which is compound type */
    let mut x: (i32, f32, u8, u16) = (500, 6.4, 1, 2344);

    let tup_0 = x.0;  // Using this we can get x.0 from;

    let tup_1 = x.1;

    let tup_2 = x.2;
    
    let tup_3 = x.3;

    x.0 = 100;// If x is mutable this ll work or compilation error

    println!("tuple ele {},{},{},{} and using var {}, {}, {}, {}", x.0, x.1, x.2, x.3, tup_0, tup_1, tup_2, tup_3);

    println!("All tuple single state: {:?}", x); // Single statment to print all tuple element. NOTE: Big tuple ll not print
    let tup_s = (1u8, 2i32); // This is new type define
    println!("tup  {}", tup_s.0);
    
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("long tup {:?}", tuple_of_tuples);
    
    let array = [1, 2, 3, 4, 5];
    println!("array {:?}", array);

    let array1 = [6; 5]; // REMEBER This is semi colon and all value assign to 5
    println!("array1 {:?}", array1);

    let array2 : [i32; 5] = [7, 2, 3, 8, 5]; // REMEBER This is semi colon
    println!("array1 {:?}", array2);
    // array comparision
    if array2 == [7,2,3,8,5] {
	println!("array2 is same");
    }
    if array2 != [8,2,3,8,5] {
	println!("array2 is not same");
    }
    for i in 0..array2.len() {
	println!("arra2 element {} = {}", i, array2[i]);
    }
    // 2D Array 2 rows and 3 coloms
    let mtx:[[f32;3]; 2] = [[1.0, 2.0, 3.0],[4.0, 5.0, 6.0]];
    println!("matrix print {:?}", mtx);
    // Printing diagonal of the matrix
    for i in 0..mtx.len() {
	for j in 0..mtx[i].len() {
	    if i == j {
		println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
	    }
	}
    }
}
