use std::collections::BTreeSet;
use std::vec::Vec;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use crate::messages::*;

fn commander(leader: Sender<Leader>, aids: BTreeSet<AcceptorId>, acceptors: Vec<Sender<Acceptor>>, replicas: Vec<Sender<Replica>>, pval: Pvalue) {
    let waitfor = aids;

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
                } else {
                    leader.send(Leader::Preempted(b));
                }
            },
        }
    }

    println!("Commander exiting");
}

fn scout(l: Sender<Leader>, aids: BTreeSet<AcceptorId>, acceptors: Vec<Sender<Acceptor>>, b: Ballot) {
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
    println!("Scount: exiting");
}

pub fn leader(id: LeaderId, aids: BTreeSet<AcceptorId>, own: Receiver<Leader>, ownt: Sender<Leader>, acceptors: Vec<Sender<Acceptor>>, replicas: Vec<Sender<Replica>>) {
    let mut active = false;
    let mut ballot_num = Ballot::make(id);
    let mut proposals = BTreeSet::new();

    let tx = ownt.clone();
    let aids0 = aids.clone();
    let acceptors0 = acceptors.clone();
    thread::spawn(move || scout(tx, aids0, acceptors0, ballot_num.clone()));
    for m in own {
        match m {
            Leader::Propose(p) => {
                if !proposals.contains(&p) {
                    proposals.insert(p.clone());
                    if active {
                        let Proposal { s: mut s, c: mut c} = p;
                        let v = Pvalue { b: ballot_num.clone(), s: s, c: c};
                        let aids0 = aids.clone();
                        let acceptors0 = acceptors.clone();
                        let replicas0 = replicas.clone();
                        let tx = ownt.clone();
                        thread::spawn(move || commander(tx, aids0, acceptors0, replicas0, v));
                    }
                }
            },
            Leader::Adopted(b, pvals) => {
                let iter = proposals.iter();
                for p in iter {
                        let Proposal { s: mut s, c: mut c} = p;
                        let v = Pvalue { b: ballot_num.clone(), s: s, c: c};
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
                }
            },
        }
    }
    println!("I'm a leader{}", &id);
}
