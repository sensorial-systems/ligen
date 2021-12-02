#![allow(unused_imports)]
use ligen::marshalling::*;
use rust::*;

#[allow(unused_unsafe)]
#[no_mangle]
pub extern "cdecl" fn print() {
	println!("Calling print");
	let result = unsafe {
		rust::print()
	};
	println!("Called print");
	result.marshal_into()
}
