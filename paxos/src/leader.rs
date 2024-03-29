use std::collections::{BTreeMap, BTreeSet};
use std::vec::Vec;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use crate::messages::*;

fn commander(leader: Sender<Leader>, aids: BTreeSet<AcceptorId>, acceptors: Vec<Sender<Acceptor>>, replicas: Vec<Sender<Replica>>, pval: Pvalue) {
    let len = aids.len();
    let mut waitfor = aids;

    let (tx, rx) = mpsc::channel();
    let acs = acceptors.iter();
    for a in acs {
        let tx_a = tx.clone();
        a.send(Acceptor::P2A(tx_a, pval));
    }
    drop(tx);

    let mut stat = true;
    while stat {
        match rx.recv().unwrap() {
            Commander::P2B(aid, b) => {
                if pval.b == b {
                    waitfor.remove(&aid);
                    if waitfor.len() < len/2 {
                        let Pvalue { b: b, s: s, c: c} = pval;
                        println!("C: Decided on ({}, {})", &s, &c);
                        for r in &replicas {
                            r.send(Replica::Decision(Proposal {s: s, c: c}));
                        }
                        stat = false;
                    }
                } else {
                    println!("C: preempted on {}", &b);
                    leader.send(Leader::Preempted(b));
                    stat = false;
                }
            },
        }
    }

    println!("Commander exiting");
}

fn scout(l: &Sender<Leader>, aids: BTreeSet<AcceptorId>, acceptors: Vec<Sender<Acceptor>>, b: Ballot) {
    println!("S: starting");

    let len = aids.len();
    let mut pvalues = BTreeSet::<Pvalue>::new();
    let mut waitfor = aids;

    let (tx, rx) = mpsc::channel();
    let acs = acceptors.iter();
    for a in acs {
        let tx_a = tx.clone();
        a.send(Acceptor::P1A(tx_a, b));
    }
    drop(tx);

    let mut stat = true;
    while stat {
        match rx.recv().unwrap() {
            Scout::P1B(id, b0, vs) => {
                if b0 == b {
                    vs.iter().map(|v| pvalues.insert(*v));
                    waitfor.remove(&id);
                    if waitfor.len() < len/2 {
                        println!("Scount: Adopted on {}", b);
                        l.send(Leader::Adopted(b, pvalues.clone()));
                        stat = false;
                    }
                } else {
                    l.send(Leader::Preempted(b));
                    stat = false;
                }
            }
        }
    }
    println!("S: exiting");
}

fn max(pvals: BTreeSet<Pvalue>) -> BTreeMap<Slot, Command> {
    let mut m = BTreeMap::new();

    pvals.iter()
        .for_each(|pval| {
            let Pvalue {b: b, s: s, c: c} = pval;
            match m.get(s) {
                Some((b0, c0)) => {
                    if b > b0 {
                        m.insert(*s, (*b, *c));
                    }
                },
                None => { m.insert(*s, (*b, *c)); },
            }
        });

    let mut n = BTreeMap::new();
    m.iter()
        .for_each(|(s, (_, c))| {
            n.insert(*s, *c);
        });
    n
}

fn update(old: &mut BTreeMap<Slot, Command>, new: BTreeMap<Slot, Command>){
    new.iter()
        .for_each(|(s, c)| {
            old.insert(*s, *c);
        })
}

pub fn leader(id: LeaderId, aids: BTreeSet<AcceptorId>, own: Receiver<Leader>, ownt: Sender<Leader>, acceptors: Vec<Sender<Acceptor>>, replicas: Vec<Sender<Replica>>) {
    let mut active = false;
    let mut ballot_num = Ballot::make(id);
    let mut proposals = BTreeMap::new();

    let tx = ownt.clone();
    let aids0 = aids.clone();
    let acceptors0 = acceptors.clone();
    thread::spawn(move || scout(&tx, aids0, acceptors0, ballot_num.clone()));
    for m in own {
        match m {
            Leader::Propose(Proposal {s: mut s, c: mut c}) => {
                match proposals.get(&s) {
                    Some(_) => (),
                    None => {
                        proposals.insert(s, c);
                        if active {
                            let v = Pvalue { b: ballot_num.clone(), s: s, c: c};
                            let aids0 = aids.clone();
                            let acceptors0 = acceptors.clone();
                            let replicas0 = replicas.clone();
                            let tx = ownt.clone();
                            thread::spawn(move || commander(tx, aids0, acceptors0, replicas0, v));
                        }
                    }
                }
            },
            Leader::Adopted(b, mut pvals) => {
                let new = max(pvals);
                update(&mut proposals, new);
                let iter = proposals.iter();
                for (s, c) in iter {
                        let v = Pvalue { b: ballot_num.clone(), s: *s, c: *c};
                        let aids0 = aids.clone();
                        let acceptors0 = acceptors.clone();
                        let replicas0 = replicas.clone();
                        let tx = ownt.clone();
                        thread::spawn(move || commander(tx, aids0, acceptors0, replicas0, v));
                }
                active = true;
            },
            Leader::Preempted(b0) => {
                if b0 > ballot_num {
                    active = false;
                    ballot_num = b0.incr(&id);
                    let tx = ownt.clone();
                    let aids0 = aids.clone();
                    let acceptors0 = acceptors.clone();
                    thread::spawn(move || scout(&tx, aids0, acceptors0, ballot_num.clone()));
                }
            },
        }
    }
    println!("I'm a leader{}", &id);
}
