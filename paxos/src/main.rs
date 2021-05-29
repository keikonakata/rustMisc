use std::collections::BTreeSet;
use std::vec::Vec;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

const WINDOW : u8 = 5;

mod messages;
mod replica;
mod leader;
mod acceptor;

fn main() {
    println!("Start!");
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let v = vec![tx1];

    let (tx2, rx2) = mpsc::channel();

    let (atx1, arx1) = mpsc::channel();
    let (atx2, arx2) = mpsc::channel();
    let (atx3, arx3) = mpsc::channel();
    let atxs = vec![atx1, atx2, atx3];

    let leaders = vec![tx2.clone()];
    let replica = thread::spawn(move || { replica::replica(0, rx, &leaders); });

    let mut aids = BTreeSet::new();
    aids.insert(0);
    aids.insert(1);
    aids.insert(2);
    let leader = thread::spawn(|| { leader::leader(0, aids, rx2, tx2, atxs, v); });

    let acceptor1 = thread::spawn(move || { acceptor::acceptor(0, arx1); });
    let acceptor2 = thread::spawn(move || { acceptor::acceptor(1, arx2); });
    let acceptor3 = thread::spawn(move || { acceptor::acceptor(2, arx3); });

    let val = messages::Replica::request();
    tx.send(val).unwrap();
    drop(tx);
    replica.join().unwrap();
    println!("Main exiting");
}

