use std::collections::BTreeSet;
use std::sync::mpsc::Receiver;

use crate::messages::*;

pub fn acceptor(id: AcceptorId, rx: Receiver<Acceptor>) {
    let mut ballot_num = Ballot::Bot;
    let mut accepted = BTreeSet::<Pvalue>::new();

    for m in rx {
        match m {
            Acceptor::P1A(tx, b) => {
                if b > ballot_num {
                    ballot_num = b;
                }
                println!("Acceptor{}: P1B on {}", &id, &ballot_num);
                tx.send(Scout::P1B(id, ballot_num.clone(), accepted.clone()));
            },
            Acceptor::P2A(tx, pval) => {
                if pval.b == ballot_num {
                    accepted.insert(pval);
                }
                println!("Acceptor{}: P2B on {}", &id, &ballot_num);
                tx.send(Commander::P2B(id, ballot_num.clone()));
            },
            Acceptor::Test => { println!("Acceptor{} Test", &id); },
        }
    }
    println!("Acceptor{}: exiting", &id);
}
