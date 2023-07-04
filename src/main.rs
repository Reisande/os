#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

use core::panic::PanicInfo;

mod vga_buffer;

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}


#[test_case]
fn trivial_assertion_2() {
    print!("trivial assertion 2 ... ");
    assert_eq!(1, 1);
    println!("[ok]");
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("Hello world!");
	
	#[cfg(test)]
    test_main();
	
    loop {
	}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
