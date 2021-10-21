use clap::{Arg, App};
use std::collections::BTreeSet;
use std::vec::Vec;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

mod messages;
mod replica;
mod leader;
mod acceptor;

// fn get_num(matches: ArgMatches<'a>, name: str, default: u8) {
//     match matches.value_of(name) {
//         None => default,
//         Some(s) =>
//             match s.parse::<u8>() {
//                 Ok(n) => n,
//                 Err(_) => panic!("failed to parse the {} param", name),
//             }
//     }
// }

fn main() {
    let matches = App::new("Paxos")
        .arg(Arg::with_name("leader")
             .short("l")
             .takes_value(true)
        )
        .arg(Arg::with_name("acceptor")
             .short("a")
             .takes_value(true)
        )
        .arg(Arg::with_name("replica")
             .short("r")
             .takes_value(true)
        )
        .get_matches();

    let num_leader =
        match matches.value_of("leader") {
            None => 1,
            Some(s) =>
                match s.parse::<u8>() {
                    Ok(n) => n,
                    Err(_) => panic!("failed to parse the leader param"),
                }
        };
    let num_replica =
        match matches.value_of("replica") {
            None => 1,
            Some(s) =>
                match s.parse::<u8>() {
                    Ok(n) => n,
                    Err(_) => panic!("failed to parse the replica param"),
                }
        };
    let num_acceptor =
        match matches.value_of("acceptor") {
            None => 3,
            Some(s) =>
                match s.parse::<u8>() {
                    Ok(n) => n,
                    Err(_) => panic!("failed to parse the acceptor param"),
                }
        };

    println!("Start!");

    let (tx_acceptors, id_acceptors) =
    {
        let mut tx_acceptors = Vec::new();
        let mut id_acceptors = BTreeSet::new();
        for i in 0..num_acceptor {
            let (tx, rx) = mpsc::channel();
            tx_acceptors.push(tx);
            id_acceptors.insert(i);
            thread::spawn(move || { acceptor::acceptor(i, rx); });
        }
        (tx_acceptors, id_acceptors)
    };

    let (mut tx_leaders, mut rx_leaders) = {
        let mut tx_leaders = Vec::new();
        let mut rx_leaders = Vec::new();
        for _ in 0..num_leader {
            let (tx, rx) = mpsc::channel();
            tx_leaders.push(tx);
            rx_leaders.push(rx);
        };
        (tx_leaders, rx_leaders)
    };

    let (replicas, tx_replicas) = {
        let mut tx_replicas = Vec::new();
        let mut replicas = Vec::new();
        for i in 0..num_replica {
            let (tx, rx) = mpsc::channel();
            tx_replicas.push(tx);
            let leaders = tx_leaders.clone();
            let r = thread::spawn(move || {replica::replica(i, rx, leaders);});
            replicas.push(r);
        };
        (replicas, tx_replicas)
    };

    for i in 0..num_leader {
        let atxs0 = tx_acceptors.clone();
        let aids0 = id_acceptors.clone();
        let v0 = tx_replicas.clone();
        let rx = rx_leaders.remove(0);
        let tx = tx_leaders.remove(0);
        thread::spawn(move || { leader::leader(i, aids0, rx, tx, atxs0, v0); });
    };

    for i in 0..3 {
        for tx in &tx_replicas {
            let val = messages::Replica::request(i);
            tx.send(val).unwrap();
        }
    }

    for r in replicas {
        r.join().unwrap();
    }

    println!("Main exiting");
}

