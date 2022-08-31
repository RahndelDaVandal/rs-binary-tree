// use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, PartialOrd, Default)]
pub struct Node<T> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}
impl<T> Node<T> 
where
    T: Copy + std::fmt::Debug
{
    pub fn new(value: T) -> Self{
        Node {
            value,
            left: None,
            right: None,
        }
    }
    pub fn get_value(&self) -> T {
        self.value
    }
    pub fn get_left(&self) -> T {
        self.left.as_ref().unwrap().get_value()
    }
    pub fn get_right(&self) -> T {
        self.right.as_ref().unwrap().get_value()
    }
    pub fn from(new_values: &[T]) -> Self {
        let (first, rest) = new_values.split_first().unwrap();
        let mut root: Node<T> = Node::new(*first);
        for value in rest {
            root.insert(*value)
        }
        root
    }
    pub fn insert(&mut self, new_value: T) {
        let mut queue: VecDeque<&mut Node<T>> = VecDeque::new();
        queue.push_front(self);
        loop {
            let Node {
            ref mut left,
            ref mut right,
            ..
            } = queue.pop_back().unwrap();
            match left {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    *left = Some(Box::new(Node::new(new_value)));
                    return ;
                }
            }
            match right {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    *right = Some(Box::new(Node::new(new_value)));
                    return ;
                }
            }
        }
    }
    pub fn left(mut self, node: Node<T>) -> Self {
        self.left = Some(Box::new(node));
        self
    }
    pub fn right(mut self, node: Node<T>) -> Self {
        self.right = Some(Box::new(node));
        self
    }
}
    // pub fn insert(&mut self, new_value: T) 
    // where
    //     T: Iterator, <T as std::iter::Iterator>::Item: std::cmp::Ord + Copy 
    // {
    //     let mut queue: VecDeque<&mut Node<T>> = VecDeque::new();
    //     queue.push_front(self);
    //
    //     loop {
    //         let Node {ref value, ref mut left, ref mut right, ..} = queue.pop_back().unwrap();
    //         match new_value.cmp(*value) {
    //             Ordering::Less => {
    //                 match left {
    //                     Some(node) => {queue.push_front(node)}
    //                     None => {*left = Some(Box::new(Node::new(new_value)))}
    //                 }
    //             },
    //             Ordering::Greater => {
    //                 match right {
    //                     Some(node) => {queue.push_front(node)}
    //                     None => {*right = Some(Box::new(Node::new(new_value)))}
    //                 }
    //             },
    //             Ordering::Equal => {
    //                 panic!("new_value: {:?} == self.value: {:?}", new_value, value)
    //             },
    //         }
    //     }
    // }
// impl From<i32> for Node<i32> {
//     fn from(value: i32) -> Self {
//         Node::new(value)
//     }
// }
// impl From<f32> for Node<f32> {
//     fn from(value: f32) -> Self {
//         Node::new(value)
//     }
// }
// impl From<i64> for Node<i64> {
//     fn from(value: i64) -> Self {
//         Node::new(value)
//     }
// }
// impl From<f64> for Node<f64> {
//     fn from(value: f64) -> Self {
//         Node::new(value)
//     }
// }

// Binary Tree Rules:
// 1 - All left nodes are less than root
// 2 - All nodes right are greater than root
// 3 - All child nodes confirm to rules 1 and 2
