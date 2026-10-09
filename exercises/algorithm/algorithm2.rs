/*
	double linked list reverse
	This problem requires you to reverse a doubly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;

        let node_ptr = Some(unsafe {
            NonNull::new_unchecked(Box::into_raw(node))
        });

        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe {
                (*end_ptr.as_ptr()).next = node_ptr
            },
        }

        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(
        &mut self,
        node: Option<NonNull<Node<T>>>,
        index: i32,
    ) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(
                    unsafe { (*next_ptr.as_ptr()).next },
                    index - 1,
                ),
            },
        }
    }

    pub fn reverse(&mut self) {
        let mut current = self.start;

        while let Some(mut ptr) = current {
            unsafe {
                let node = ptr.as_mut();
                let next = node.next;

                node.next = node.prev;
                node.prev = next;

                current = next;
            }
        }

        std::mem::swap(&mut self.start, &mut self.end);
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(
                f,
                "{}, {}",
                self.val,
                unsafe { node.as_ref() }
            ),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list = LinkedList::<String>::new();
        list.add("A".to_string());
        list.add("B".to_string());
        list.add("C".to_string());
        assert_eq!(3, list.length);
    }

    #[test]
    fn test_reverse_linked_list_1() {
        let mut list = LinkedList::new();
        let original = [2, 3, 5, 11, 9, 7];
        let expected = [7, 9, 11, 5, 3, 2];

        for x in original {
            list.add(x);
        }

        list.reverse();

        for (i, x) in expected.iter().enumerate() {
            assert_eq!(*x, *list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_linked_list_2() {
        let mut list = LinkedList::new();
        let original = [34, 56, 78, 25, 90, 10, 19, 34, 21, 45];
        let expected = [45, 21, 34, 19, 10, 90, 25, 78, 56, 34];

        for x in original {
            list.add(x);
        }

        list.reverse();

        for (i, x) in expected.iter().enumerate() {
            assert_eq!(*x, *list.get(i as i32).unwrap());
        }
    }
}
