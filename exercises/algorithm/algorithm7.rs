/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        let result = self.data.pop();
        if result.is_some() {
            self.size -= 1;
        }
        result
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.last_mut()
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<'_, T> {
        Iter {
            stack: self.data.iter().collect(),
        }
    }

    fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            stack: self.data.iter_mut().collect(),
        }
    }
}

struct IntoIter<T>(Stack<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

fn bracket_match(bracket: &str) -> bool {
    let mut stack = Stack::new();

    for ch in bracket.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),

            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }

            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }

            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }

            _ => {}
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bracket_matching_1() {
        assert_eq!(bracket_match("(2+3){func}[abc]"), true);
    }

    #[test]
    fn bracket_matching_2() {
        assert_eq!(bracket_match("(2+3)*(3-1"), false);
    }

    #[test]
    fn bracket_matching_3() {
        assert_eq!(bracket_match("{{([])}}"), true);
    }

    #[test]
    fn bracket_matching_4() {
        assert_eq!(bracket_match("{{(}[)]}"), false);
    }

    #[test]
    fn bracket_matching_5() {
        assert_eq!(bracket_match("[[[]]]]]]]]]"), false);
    }

    #[test]
    fn bracket_matching_6() {
        assert_eq!(bracket_match(""), true);
    }
}
