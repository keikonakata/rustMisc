use std::collections::BTreeSet;
use std::sync::mpsc::Receiver;
use std::thread;

use crate::messages::{Command, Proposal, Replica};

fn propose(req: &BTreeSet<Command>) {
    let window = 5;
    let mut slot_in = 1;
    let slot_out = 1;
    let proposals = BTreeSet::<Proposal>::new();
    let decisions = BTreeSet::<Proposal>::new();
    let mut leaders = BTreeSet::<u8>::new();

    while slot_in < slot_out + window && req.is_empty() {
        if decisions.iter().all(|x| x.s != slot_in) {
        }
        slot_in = slot_in + 1;
    };

    println!("Bye");
}

pub fn replica(rx: Receiver::<Replica>) {
    let mut req = BTreeSet::<Command>::new();
    for received in rx {
        match received {
            Replica::Request(c) => {
                req.insert(c);
                propose(&req);
            }
            Replica::Decision(_) => (),
        }
        println!("Replica: received");
    }
    println!("Replica: Bye");
}

