//use core::any;

pub mod front_of_house {
    pub mod hosting {
	pub fn add_to_waitlist() {
	    println!("Inside add_to_waitlist()");
	}
	fn seat_at_table() {
	    println!("Inside seat_at_table()");
	}
    }
    mod serving {
	fn take_order() {
	    println!("Inside take_order()");
	}
	fn take_payment() {
	    println!("Inside take_payment()");
	}
	fn serve_order() {
	    println!("Inside serve_order()");
	}
    }
}

pub fn eat_at_resturent() {
    crate::front_of_house::hosting::add_to_waitlist(); //Asolute path
    front_of_house::hosting::add_to_waitlist(); //Asolute path
}
