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

    pub fn incr(&self, id: &u8) -> Ballot {
        match self {
            Ballot::N(i, _) => Ballot::N(i+1, *id),
            Ballot::Bot => panic!(),
        }
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

pub type Slot = u8;

pub type AcceptorId = u8;

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Op { Step(i8) }

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result  {
        match self {
            Op::Step(i) =>  write!(f, "Step{}", i),
        }
    }
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Command { k: ClientId, c: CommandId, pub o: Op}

impl Command {
    pub fn make(i: i8) -> Command {
        Command {
            k: 0,
            c: 0,
            o: Op::Step(i),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result  {
        let Command { k:k, c:c, o:o } = self;
        write!(f, "({}, {}, {})", k, c, o)
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
    pub fn request(i: i8) -> Replica {
        Replica::Request(Command::make(i))
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
    Test,
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
