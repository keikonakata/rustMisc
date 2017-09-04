use std::cmp::Ordering;

struct Avl<K, V> {
    key: K,
    v: V,
    height: u32,
    right: AvlTree<K,V>,
    left: AvlTree<K,V>
}

type AvlTree<K, V> = Option<Box<Avl<K,V>>>;

enum Path<K, V> {
    Top,
    LNode { t: AvlTree<K, V>, key: K, v: V, height: u32, p: Box<Path<K, V>> },
    RNode { p: Box<Path<K, V>>, key: K, v: V, height: u32, t: AvlTree<K, V> }
}

type Location<K, V> = (AvlTree<K, V>, Path<K, V>); 

fn zip<K, V>(lc: Location<K, V>) -> AvlTree<K, V> {
    match lc {
        (tr, Path::Top) => tr,
        (tr, Path::LNode{t, key, v, height, p}) =>
            zip((Some(Box::new(Avl{key: key, v: v, height: height, right: tr, left: t})), *p)),
        (tr, Path::RNode{t, key, v, height, p}) =>
            zip((Some(Box::new(Avl{key: key, v: v, height: height, right: t, left: tr})), *p)),
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
            let Avl{key, v, height, right, left} = t_;
            (right, Path::RNode{p: Box::new(p), key:key, v: v, height: height, t: left})
        }
    }
}

fn go_left<K, V>(loc: Location<K, V>) -> Location<K, V> {
    match loc {
        (None, _) => panic!("cannot go left"),
        (Some(t), p) => {
            let t_ = *t;
            let Avl{key, v, height, right, left} = t_;
            (left, Path::RNode{p: Box::new(p), key:key, v: v, height: height, t: right})
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

fn insert<K: Ord, V>(key: K, v: V, t: AvlTree<K, V>) {
    match search(&key, (t, Path::Top)) {
        (None, p) =>
            zip((Some(Box::new(Avl{key: key, v: v, height: 0, right: None, left: None})), p)),
        (Some(_), _) => panic!("key already exists")
    };
}

fn rotate_right<K, V>(mut t: Box<Avl<K, V>>) -> Box<Avl<K, V>> {
    let mut l = t.left.take().expect("Left child is null");
    let lr = l.right.take();  
    t.left = lr;
    l.right = Some(t);
    l
}

fn rotate_left<K, V>(mut t: Box<Avl<K, V>>) -> Box<Avl<K, V>> {
    let mut r = t.right.take().expect("Right child is null");
    let rl = r.left.take();  
    t.right = rl;
    r.left = Some(t);
    r
}

fn print(t: &AvlTree<i32, i32>) {
    match t {
        None => println("."),
        Some(ref t) => {
            match t {
                Avl{key, v, height, right, left} => {
                    println("key: {}, v: {}, height: {}", key, v, height);
                    print(&right); print(&left)
                }
            }
        }
    }
}

fn main() {
    let t0 = insert(6, 1, None);
    print(&t0)
}


