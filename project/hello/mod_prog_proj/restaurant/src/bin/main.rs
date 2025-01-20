//use core::any;
use restaurant::front_of_house::hosting;

fn main() {
    println!("Welcome to the module");
    use restaurant::eat_at_resturent;
    eat_at_resturent();
    //use restaurant::front_of_house::hosting::add_to_waitlist()
    hosting::add_to_waitlist();
    //hosting::add_to_waitlist();
    //hosting::add_to_waitlist();
} 
