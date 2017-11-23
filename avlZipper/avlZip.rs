use std::cmp::Ordering;

struct Avl<K, V> {
    key: K,
    v: V,
    balance: i8, // -1 -> left biased, +1 -> right biased
    right: AvlTree<K,V>,
    left: AvlTree<K,V>
}

type AvlTree<K, V> = Option<Box<Avl<K,V>>>;

enum Path<K, V> {
    Top,
    LNode { t: AvlTree<K, V>, key: K, v: V, balance: i8, p: Box<Path<K, V>> },
    RNode { p: Box<Path<K, V>>, key: K, v: V, balance: i8, t: AvlTree<K, V> }
}

type Location<K, V> = (AvlTree<K, V>, Path<K, V>); 

fn zip<K, V>(lc: Location<K, V>) -> AvlTree<K, V> {
    match lc {
        (tr, Path::Top) => tr,
        (tr, Path::LNode{t, key, v, balance, p}) =>
            zip((Some(Box::new(Avl{key: key, v: v, balance: balance, right: tr, left: t})), *p)),
        (tr, Path::RNode{t, key, v, balance, p}) =>
            zip((Some(Box::new(Avl{key: key, v: v, balance: balance, right: t, left: tr})), *p)),
    }
}

fn is_null<K, V>(loc: & Location<K, V>) -> bool {
    match *loc {
        (None, _) => true,
        _ => false
    }
}

fn cmp<K: Ord, V>(key: & K, loc: & Location<K, V>) -> Ordering {
    match *loc {
        (None, _) => panic!("cannot compare with None"),
        (Some(ref t), _) => t.key.cmp(key)
    }
}

fn go_right<K, V>(loc: Location<K, V>) -> Location<K, V> {
    match loc {
        (None, _) => panic!("cannot go right"),
        (Some(t), p) => {
            let t_ = *t;
            let Avl{key, v, balance, right, left} = t_;
            (right, Path::LNode{t: left, key:key, v: v, balance: balance, p: Box::new(p)})
        }
    }
}

fn go_left<K, V>(loc: Location<K, V>) -> Location<K, V> {
    match loc {
        (None, _) => panic!("cannot go left"),
        (Some(t), p) => {
            let t_ = *t;
            let Avl{key, v, balance, right, left} = t_;
            (left, Path::RNode{p: Box::new(p), key:key, v: v, balance: balance, t: right})
        }
    }
}

fn search<K: Ord, V>(key: &K, loc: Location<K, V>) -> Location<K, V> {
    if is_null(&loc) {
        loc
    } else {
        match cmp(key, &loc) {
            Ordering::Less => search(key, go_left(loc)),
            Ordering::Equal => loc,
            Ordering::Greater => search(key, go_right(loc))
        }
    }
}

fn insert<K: Ord, V>(key: K, v: V, t: AvlTree<K, V>) -> AvlTree<K, V>{
    match search(&key, (t, Path::Top)) {
        (None, p) =>
            zip((Some(Box::new(Avl{key: key, v: v, balance: 0, right: None, left: None})), p)),
        (Some(_), _) => panic!("key already exists")
    }
}

fn rotate_right<K, V>(mut t: Box<Avl<K, V>>) -> Box<Avl<K, V>> {
    let mut l = t.left.take().expect("Left child is null");
    // adjust balance
    if l.balance == -1 {
        t.balance = 0;
        l.balance = 0
    } else if l.balance == 0 {
        t.balance = -1;
        l.balance = 1;
    } else {
        panic!("impossible")
    };
    let lr = l.right.take();  
    t.left = lr;
    l.right = Some(t);
    l
}

fn rotate_left<K, V>(mut t: Box<Avl<K, V>>) -> Box<Avl<K, V>> {
    let mut r = t.right.take().expect("Right child is null");
    // adjust balance
    if r.balance == 1 {
        t.balance = 0;
        r.balance = 0
    } else if r.balance == 0 {
        t.balance = 1;
        r.balance = -1;
    } else {
        panic!("impossible")
    };
    let rl = r.left.take();  
    t.right = rl;
    r.left = Some(t);
    r
}

fn print(t: &AvlTree<i32, i32>) {
    match *t {
        None => println!("."),
        Some(ref n) => {
            let Avl{key, v, balance, ref right, ref left} = **n;
            println!("key: {}, v: {}, balance: {}", key, v, balance);
            print!("R: "); print(right);
            print!("L: "); print(left)
        }
    }
}

fn main() {
    let t0 = insert(1, 4, insert(8, 2, insert(6, 1, None)));
    print(&t0)
}
