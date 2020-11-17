use std::sync::mpsc;
use std::thread;
use rand::Rng;

fn main() {
    let (tx, rx) = mpsc::channel();
    const N: usize = 100; 
    let mut xs: [i32; N] = [0; N];

    for i in 0..10 {
        xs[i] = rand::thread_rng().gen_range(1, 101);
    }

    let mut s: i32 = 0;

    let tx1 = mpsc::Sender::clone(&tx);
    let first = thread::spawn(move || {
        let mut sum: i32 = 0;
        for i in 0..(N/2){
        	sum += xs[i];
        }

        tx1.send(sum).unwrap();
        drop(tx1);
    });

    let sec = thread::spawn(move || {
        let mut sum: i32 = 0;
        for i in (N/2)..N{
        	sum += xs[i];
        }

        tx.send(sum).unwrap();
        drop(tx);
    });
    first.join().unwrap();
    sec.join().unwrap();

    for received in rx {
        s = s + received;
    }
    

    println!("{}", s);
}