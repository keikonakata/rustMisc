use std::collections::BTreeSet;
use std::sync::mpsc::Sender;
use std::fmt;

pub type LeaderId = u8;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Copy, Clone)]
pub enum Ballot {
    Bot,
    N(u8, LeaderId),
}

impl Ballot {
    pub fn make(id: u8) -> Ballot {
        Ballot::N(0, id)
    }
}

impl fmt::Display for Ballot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result  {
        match self {
            Ballot::Bot =>  write!(f, "bot"),
            Ballot::N(i, j) => write!(f, "({}, {})", i, j),
        }
    }
}

type ClientId = u8;

type CommandId = u8;

type Slot = u8;

pub type AcceptorId = u8;

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Op { Step }

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Command { k: ClientId, c: CommandId, o: Op}

impl Command {
    pub fn make() -> Command {
        Command {
            k: 0,
            c: 0,
            o: Op::Step,
        }
    }
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Proposal { pub s: u8, pub c: Command}

pub enum Replica {
    Request(Command),
    Decision(Proposal),
}

impl Replica {
    pub fn request() -> Replica {
        Replica::Request(Command::make())
    }
}

pub enum Leader {
    Adopted(Ballot, BTreeSet<Pvalue>),
    Preempted(Ballot),
    Propose(Proposal),
}

pub enum Commander {
    P2B(AcceptorId, Ballot),
}

pub enum Acceptor {
    P1A(Sender<Scout>, Ballot),
    P2A(Sender<Commander>, Pvalue),
}

pub enum Scout {
    P1B(AcceptorId, Ballot, BTreeSet<Pvalue>),
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Pvalue {
    pub b: Ballot,
    pub s: Slot,
    pub c: Command,
}
