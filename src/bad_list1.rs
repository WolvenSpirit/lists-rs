//use std::rc::Rc;
use std::ops::Deref;
use std::ops::DerefMut;

struct Node<'a, T> {
    value: T,
    next_value: Option<'a Box<Node<'a,T>>>, 
}

impl Deref for Node<'_,i32> {
    type Target<'r> = Node<'r, i32>;
    fn deref(&self) -> &Self::Target {
        return self;
    }
}

impl DerefMut for Node<'_,i32> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return self;
    }
}

impl AsMut for Node<'_,i32> {
    fn as_mut(&mut self) -> &mut Node<i32> {
        todo!()
    }
}
pub fn m() {
    let mut n2 = Box::new(Node{value:2, next_value: None});
    let mut n = Box::new(Node{value: 1, next_value: None });
    let n3 = n2.as_mut(); // = Some(n);
    //n.next_value = Some(n2);
    //println!("{}", n.value);
    // println!("{}", n.next_value.clone().unwrap().value);
}
