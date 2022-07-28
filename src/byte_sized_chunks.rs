use std::num::Wrapping;

pub fn byte_sized_chunks() {
    let mut counter : u8 = 0;
    loop {
        println!("{}", counter);
        counter += 1;
    }
}

pub fn byte_sized_chunks_wrapping() {
	let mut counter = Wrapping(0i8);
	    loop {
	        println!("{}", counter);
	        counter +=Wrapping(1i8);
	    }
}

pub fn byte_sized_chunks_checked() {
    let mut counter :i8 = 0;
    let mut step = 1;

    loop {
        println!("{}", counter);

        if let Some(x) = counter.checked_add(step) {
            counter = x;
        } else {
            counter = 0;
            step *= 2;
        }
    }
}