

pub mod stack {
    pub struct TwoStack<T: Clone> {
        pub one: Option<T>,
        pub two: Option<T>,
        pub len: usize,
    }

    impl<T: Clone> TwoStack<T> {
        pub fn new() -> TwoStack<T> {
            TwoStack {one: None, two: None, len: 0}
        }
        pub fn push(&mut self, item: T) {

            match &self.one {
                Some(t) => {
                    self.two = Some(t.clone());
                    self.one = Some(item);
                    self.len = 2;
                },
                None => {
                    self.one = Some(item);
                    self.len = 1;
                }
            };

        }
        pub fn pop(&mut self) -> Option<T> {
            match &self.one {
                Some(t) => {
                    let a = t.clone();
                    match &self.two {
                        Some(s) => {
                            self.one = Some(s.clone());
                            self.len = 1;
                        },
                        None => {
                            self.one = None;
                            self.len = 0;
                        }
                    }
                    return Some(a);
                },
                None => {
                    self.len = 0;
                    return None;
                }
            }
        }

    }
}

#[cfg(test)]
mod stack_tests {
    use super::stack::TwoStack;

    #[test]
    fn empty_item() {
        let mut s: TwoStack<u8> = TwoStack::new();

        assert_eq!(s.len, 0);

    }

    #[test]
    fn one_item() {
        let mut s: TwoStack<i64> = TwoStack::new();
        let a = i64::from(12);
        s.push(a);

        assert_eq!(s.len, 1);
    }

    #[test]
    fn two_items() {
        let mut s: TwoStack<i64> = TwoStack::new();
        let a = i64::from(12);
        let b = i64::from(23);
        s.push(a);
        s.push(b);

        assert_eq!(s.one, Some(23));
        assert_eq!(s.two, Some(12));
        assert_eq!(s.len, 2);
    }

    #[test]
    fn three_items() {
        let mut s: TwoStack<i64> = TwoStack::new();
        let a = i64::from(-12);
        let b = i64::from(34);
        let c = i64::from(123);

        s.push(a);
        s.push(b);
        s.push(c);

        assert_eq!(s.len, 2);
        assert_eq!(s.one, Some(123));
        assert_eq!(s.two, Some(34));
    }

    #[test]
    fn pop_one() {
        let mut s: TwoStack<i64> = TwoStack::new();
        let a = i64::from(23);
        let b = i64::from(12);
        s.push(a);

        let c = s.pop();
        assert_eq!(c, Some(23));
        assert_eq!(s.len, 0);
    }
}
