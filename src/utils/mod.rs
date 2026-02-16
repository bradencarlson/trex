

pub mod stack {
    pub struct TwoStack<'a,T> {
        pub one: Option<&'a T>,
        pub two: Option<&'a T>,
        pub len: usize,
    }

    impl<'a,T> TwoStack<'a,T> {
        pub fn new() -> TwoStack<'a,T> {
            TwoStack {one: None, two: None, len: 0}
        }
        pub fn push(&mut self, item: &'a T) {
            self.two = self.one;
            self.one = Some(item);
            if let Some(_a) = self.two {
                self.len = 2;
            } else {
                self.len = 1;
            }
        }
        pub fn pop(&mut self) -> Option<&'a T> {
            let a = self.one;
            self.one = self.two;
            self.two = None;
            if let Some(_b) = self.one {
                self.len = 1; 
            } else {
                self.len = 0;
            }
            a
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
        s.push(&a);

        assert_eq!(s.len, 1);
    }

    #[test]
    fn two_items() {
        let mut s: TwoStack<i64> = TwoStack::new();
        let a = i64::from(12);
        let b = i64::from(23);
        s.push(&a);
        s.push(&b);

        assert_eq!(s.one, Some(b).as_ref());
        assert_eq!(s.two, Some(a).as_ref());
        assert_eq!(s.len, 2);
    }

    #[test]
    fn three_items() {
        let mut s: TwoStack<i64> = TwoStack::new();
        let a = i64::from(-12);
        let b = i64::from(34);
        let c = i64::from(123);

        s.push(&a);
        s.push(&b);
        s.push(&c);

        assert_eq!(s.len, 2);
        assert_eq!(s.one, Some(c).as_ref());
        assert_eq!(s.two, Some(b).as_ref());
    }

    #[test]
    fn pop_one() {
        let mut s: TwoStack<i64> = TwoStack::new();
        let a = i64::from(23);
        let b = i64::from(12);
        s.push(&a);

        let c = s.pop();
        assert_eq!(c, Some(23).as_ref());
        assert_eq!(s.len, 0);
    }
}
