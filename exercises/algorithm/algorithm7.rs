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
        if self.size > 0 {
            self.size -= 1;
            self.data.pop()
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.last_mut()
    }

    // 其余迭代器方法保持不变...
}

fn bracket_match(bracket: &str) -> bool {
    let mut stack = Stack::new();
    
    for c in bracket.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => (),
        }
    }
    
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bracket_matching_1() {
        let s = "(2+3){func}[abc]";
        assert!(bracket_match(s));
    }

    #[test]
    fn bracket_matching_2() {
        let s = "(2+3)*(3-1";
        assert!(!bracket_match(s));
    }

    #[test]
    fn bracket_matching_3() {
        let s = "{{([])}}";
        assert!(bracket_match(s));
    }

    #[test]
    fn bracket_matching_4() {
        let s = "{{(}[)]}";
        assert!(!bracket_match(s));
    }

    #[test]
    fn bracket_matching_5() {
        let s = "[[[]]]]]]]]]";
        assert!(!bracket_match(s));
    }

    #[test]
    fn bracket_matching_6() {
        let s = "";
        assert!(bracket_match(s));
    }
}