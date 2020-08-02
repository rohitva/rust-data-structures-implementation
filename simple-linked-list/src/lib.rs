/// RustList provide simple LinkedList structure functionality.
// TODO: Make me generic
// TODO: Make me faster. Currently add is O(N)
// TODO: Benchmark me
#[derive(Debug)]
struct RustList<T> {
    value: T,
    next_node: Option<Box<RustList<T>>>,
}

impl<T> RustList<T> {
    /// Create a new list.
    fn new(value: T) -> Self {
        RustList {
            value,
            next_node: None,
        }
    }

    /// Create a new value to the end of the list.
    fn add(&mut self, value: T) {
        self.next_node = Some(Box::new(RustList::new(value)));
    }
}

#[cfg(test)]
mod tests {
    use crate::RustList;

    #[test]
    fn create_new_with_int() {
        let mut new_list = RustList::new(10);
        new_list.add(15);
        new_list.add(20);
        new_list.add(40);
        println!("new_list {:?}", new_list);
    }

    #[test]
    fn create_new_with_string() {
        let mut new_list = RustList::new("testStringOne");
        new_list.add("testStringTwo");
        new_list.add("testStringThree");
        new_list.add("testStringFour");
        println!("new_list {:?}", new_list);
    }
}
