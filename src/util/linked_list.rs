#[derive(PartialEq, Eq, Debug)]
pub struct ListNodef{
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Listnode { next: None, val }
    }
}

// helper function for test
/pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    lt mut current = None;
    for &v in vec.iter().rev() {
        let mut node = Listnode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

#[macro_export]
macro_rules! linked {
    ($($e:expr),*) => {to_list(vec![$($e.to_owned()), *])};
    ($($e:expr,)*) => {to_list(vec![$($e.to_owned()), *])};
}

