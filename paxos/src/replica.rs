use std::collections::{BTreeMap, BTreeSet};
use std::vec::Vec;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::messages::*;

fn propose(req: &mut BTreeSet<Command>, proposals: &mut BTreeMap::<u8, Command>, decisions: & BTreeMap::<u8, Command>, leaders: &Vec<Sender<Leader>>, slot_out: &u8, slot_in: &mut u8) {

    while *slot_in < slot_out + crate::WINDOW && !req.is_empty() {
        match decisions.get(&*slot_in) {
            Some(_) => (),
            None => {
                match req.iter().next() {
                    Some(&c) => {
                        req.remove(&c);
                        proposals.insert(*slot_in, c);
                        for l in leaders {
                            l.send(Leader::Propose(Proposal {s: *slot_in, c: c}));
                        }
                    },
                    None => panic!(),
                }
            },
        }
        *slot_in = *slot_in + 1;
    };
}

pub fn perform() {
}

pub fn replica(id: u8, rx: Receiver::<Replica>, leaders: &Vec<Sender<Leader>>) {
    let mut slot_in = 1;
    let mut req = BTreeSet::<Command>::new();
    let mut decisions = BTreeMap::<u8, _>::new();
    let mut proposals = BTreeMap::<u8, _>::new();
    let mut slot_out = 1;

    for received in rx {
        match received {
            Replica::Request(c) => {
                req.insert(c);
            }
            Replica::Decision(p) => {
                let Proposal {s: s, c: c} = p;
                decisions.insert(s, c);
                match decisions.get(&slot_out) {
                    Some(c0) => {
                        match proposals.get(&slot_out) {
                            Some(c1) => { if c0 != c1 { req.insert(*c1); } },
                            None => (),
                        }
                     perform();
                    },
                    None => (),
                }
            },
        }
        propose(&mut req, &mut proposals, &decisions, leaders, &slot_out, &mut slot_in);
        println!("R{}: received", &id);
    }
    println!("R{}: Bye", &id);
}

