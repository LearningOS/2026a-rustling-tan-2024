/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
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
}

impl<T: Ord> LinkedList<T> {
    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self {
        let mut result = Self::new();

        while let (Some(a), Some(b)) = (list_a.start, list_b.start) {
            let take_a = unsafe {
                (*a.as_ptr()).val <= (*b.as_ptr()).val
            };

            let node = if take_a {
                let next = unsafe { (*a.as_ptr()).next };
                list_a.start = next;
                list_a.length -= 1;
                a
            } else {
                let next = unsafe { (*b.as_ptr()).next };
                list_b.start = next;
                list_b.length -= 1;
                b
            };

            unsafe {
                (*node.as_ptr()).next = None;
            }

            if let Some(end) = result.end {
                unsafe {
                    (*end.as_ptr()).next = Some(node);
                }
            } else {
                result.start = Some(node);
            }

            result.end = Some(node);
            result.length += 1;
        }

        let remaining = if list_a.start.is_some() {
            &mut list_a
        } else {
            &mut list_b
        };

        if let Some(start) = remaining.start {
            if let Some(end) = result.end {
                unsafe {
                    (*end.as_ptr()).next = Some(start);
                }
            } else {
                result.start = Some(start);
            }

            result.end = remaining.end;
            result.length += remaining.length;
        }

        list_a.start = None;
        list_b.start = None;
        result
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
    fn test_merge_linked_list_1() {
        let mut a = LinkedList::new();
        let mut b = LinkedList::new();

        for x in [1, 3, 5, 7] {
            a.add(x);
        }
        for x in [2, 4, 6, 8] {
            b.add(x);
        }

        let mut c = LinkedList::merge(a, b);
        let expected = [1, 2, 3, 4, 5, 6, 7, 8];

        for (i, x) in expected.iter().enumerate() {
            assert_eq!(*x, *c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut a = LinkedList::new();
        let mut b = LinkedList::new();

        for x in [11, 33, 44, 88, 89, 90, 100] {
            a.add(x);
        }
        for x in [1, 22, 30, 45] {
            b.add(x);
        }

        let mut c = LinkedList::merge(a, b);
        let expected = [1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for (i, x) in expected.iter().enumerate() {
            assert_eq!(*x, *c.get(i as i32).unwrap());
        }
    }
}
