const SIZE: usize = 1000;
const PART: usize = SIZE / 4;
static mut a: [[i32; SIZE]; SIZE] = [[0 as i32; SIZE]; SIZE];
static mut b: [[i32; SIZE]; SIZE] = [[0 as i32; SIZE]; SIZE];
static mut c: [[i32; SIZE]; SIZE] = [[0 as i32; SIZE]; SIZE];
use std::thread;

fn Thread(of: usize) {         //filling in matrices
	unsafe {
		for i in of..(of + PART) {
	        for j in 0..SIZE {
	            a[i][j] = 20;
	            b[i][j] = 15;
	            c[i][j] = a[i][j] + b[i][j];
	        }
	    }
	}
}

fn main() {

	let of1: usize = 0;
    let of2: usize = PART;
    let of3: usize = 2 * PART;
    let of4: usize = 3 * PART;

    let thread1 = thread::spawn(move || {
    	Thread(of1);
    });
    let thread2 = thread::spawn(move || {
    	Thread(of2);
    });
    let thread3 = thread::spawn(move || {
    	Thread(of3);
    });
    let thread4 = thread::spawn(move || {
    	Thread(of4);
    });

    thread1.join();
    thread2.join();
    thread3.join();
    thread4.join();

}
