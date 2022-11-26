use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: usize,
    // node_point: Option<Rc<RefCell<Node>>>,
    node_point: Option<Weak<RefCell<Node>>>,
}

fn main() {
    let n2 = Rc::new(RefCell::new(Node {
        value: 2,
        node_point: None,
    }));

    let n1 = Rc::new(RefCell::new(Node {
        value: 1,
        // node_point: Some(Rc::clone(&n2)),
        node_point: Some(Rc::downgrade(&n2)),
    }));

    // n2.borrow_mut().node_point = Some(Rc::clone(&n1));
    n2.borrow_mut().node_point = Some(Rc::downgrade(&n1));
    // dbg!(Rc::strong_count(&n1));
    // dbg!(Rc::strong_count(&n2));
    dbg!(Rc::weak_count(&n1));
    dbg!(Rc::weak_count(&n2));
    std::mem::drop(n1);

    // Rc::strong_count(&n2) is still 2 (instead of down to 1) even after calling drop on n1,
    // But could not be freed because n2 was holding a reference to n1.
    // dbg!(Rc::strong_count(&n2));
    dbg!(Rc::weak_count(&n2));

    // Printing out the cyclical infinite reference between n1 and n2.
    // Non detectable at compile time but
    // breaking at runtime because of Rc+RefCell combination
    // (even after calling drop on n1) thus causing the memory leak!.
    dbg!(&n2);

    // The next debug will who the following with
    // node_point = None after upgrade because n1 was dropped:
    // std::mem::drop(n1)
    //
    //[src\main.rs:27] Rc::weak_count(&n1) = 1
    // [src\main.rs:28] Rc::weak_count(&n2) = 1
    // [src\main.rs:34] Rc::weak_count(&n2) = 0
    // [src\main.rs:40] &n2 = RefCell {
    //     value: Node {
    //         value: 2,
    //         node_point: Some(
    //             (Weak),
    //         ),
    //     },
    // }
    // [src\main.rs:41] n2.borrow().node_point.as_ref().unwrap().upgrade() = None
    dbg!(n2.borrow().node_point.as_ref().unwrap().upgrade());
}