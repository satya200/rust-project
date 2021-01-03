fn main() {
    /*This println is not a function it is macro. "!" symbol represent
     * that use println macro insted of println function and ln means it will 
     * add mew line at the end */
    println!("Hellow World\nHappy New Year 2021");
    /* Diff bet println and print is print ll not add new line at the end. You need 
     * add new line manually at last below is the example */
    print!("Hellow World\nHappy New Year 2021\n");
    println!("{0}, This is {1}", "Satya", "Hari");
    // Below is named argument. Below subject and verb will not print on the consol
    println!("{subject}, {verb}", subject = "Satya", verb = "Hari");
    // :b represnts binary of 5. This :b is called traits. :? is use for debug traits
    println!("{} is {:5}", 1, 5);
    // right aignment with 5 spaces
    println!("{number:>width$}", number = 1, width = 6);
    // right aignment with 5 spaces and pad extra zero
    println!("{number:>0width$}", number = 1, width = 6);
    // Left aignment with 5 spaces
    println!("{number:<width$}", number = 1, width = 6);
    eprintln!("Use for print in stderror");
    eprint!("Use for print in stderror\n");
}
