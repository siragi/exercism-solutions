#[allow(dead_code)]
use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, element: T) {
        let new_node = Node::new(element, self.head.take());
        self.head = Some(Box::new(new_node));
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.length -= 1;
                let current_node = *node;
                self.head = current_node.next;
                Some(current_node.data)
            }
            None => None,
        }
        // Functional style works too!
        // self.head.take().map(|node| {
        //     self.head = node.next;
        //     self.length -= 1;
        //     node.data
        // })
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head {
            Some(ref node) => {
                let current_element = &node.data;
                Some(current_element)
            }
            None => None,
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T>
// where
    //     T: Clone,
    {
        let mut v: Vec<T> = self.into();
        v.reverse();
        v.drain(..).collect()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for element in iter {
            list.push(element)
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

/* impl<T> Into<Vec<T>> for SimpleLinkedList<T>
where
    T: Clone,
{
    fn into(self) -> Vec<T> {
        let mut vec: Vec<T> = Vec::new();
        let mut current_node = self.head;

        loop {
            match current_node {
                Some(boxed_node) => {
                    let node = boxed_node.as_ref().clone();
                    vec.push(node.data);
                    current_node = boxed_node.next;
                }
                None => {
                    break;
                }
            }
        }
        vec.reverse();
        vec
    }
} */
impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut v = Vec::new();
        while let Some(data) = self.pop() {
            v.push(data);
        }
        v.reverse();
        v
    }
}

// #[derive(Clone)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Self { data, next }
    }
}
