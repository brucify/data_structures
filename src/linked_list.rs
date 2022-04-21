#[derive(Debug, Clone, PartialEq)]
struct Node {
    value: String,
    next: Option<Box<Node>>
}

impl Node {
    fn new(value: String, next: Option<Box<Node>>) -> Box<Node> {
        Box::new(Node { value, next })
    }

    fn value(&mut self, value: String) {
        self.value = value;
    }

    fn next(&mut self, next: Option<Box<Node>>) {
        self.next = next;
    }

    fn find(&self, value: &String, acc: usize) -> Option<usize> {
        if self.value.eq(value) {
            Some(acc)
        } else {
            self.next.as_ref()
                .map(|node| node.find(value, acc+1))
                .flatten()
        }
    }

    fn maybe_at_index(&self, index: usize) -> Option<&Box<Node>> {
        // start from checking self.next (therefore 1..index)
        // if next is Some(node), map to Some(node.next),
        // while keeping the None
        (1..index).fold(self.next.as_ref(), |acc, _| {
            acc.map(|n| n.next.as_ref()).flatten()
        })
    }

    fn maybe_at_index_mut(&mut self, index: usize) -> Option<&mut Box<Node>> {
        // start from checking self.next (therefore 1..index)
        // if next is Some(node), map to Some(node.next),
        // while keeping the None
        (1..index).fold(self.next.as_mut(), |acc, _| {
            acc.map(|n| n.next.as_mut()).flatten()
        })
    }

    fn at_index_mut(&mut self, index: usize) -> &mut Node {
        // start from &mut self (therefore 0..index)
        // if next is None, keep current
        // if next is Some(node), return &mut node
        // always return one &mut Node
        (0..index).fold(self, |acc, _| {
            match acc.next.as_ref() {
                None => acc,
                Some(_) => acc.next.as_deref_mut().unwrap()
            }
        })
    }

}

#[derive(Debug)]
pub struct List {
    first: Option<Box<Node>>
}

impl List {
    pub fn new() -> List {
        List{first: None}
    }

    pub fn at(&self, index: usize) -> Option<&String> {
        match (self.first.as_ref(), index) {
            (None, _) =>
                None,
            (Some(first), 0) =>
                Some(&first.value),
            (Some(first), index) =>
                first.maybe_at_index(index).map(|n| &n.value)
        }

    }

    pub fn find(&self, value: &String) -> Option<usize> {
        self.first.as_ref()
            .map(|node| node.find(value, 0))
            .flatten()
    }

    pub fn insert(&mut self, index: usize, value: String) {
        match (self.first.as_mut(), index) {
            (None, _) | (_, 0) =>
                self.first = Some(Node::new(value, self.first.clone())),
            (Some(first), index) => {
                let previous = first.at_index_mut(index-1);
                let new = Node::new(value, previous.next.clone());
                previous.next(Some(new));
            }
        }
    }

    pub fn delete(&mut self, index: usize) {
        match (self.first.as_mut(), index) {
            (None, _) =>
                {}
            (Some(first), 0) =>
                self.first = first.next.clone(),
            (Some(first), index) => {
                let previous = first.at_index_mut(index-1);
                let next = previous.next.as_ref()
                    .map(|target|target.next.clone())
                    .flatten();
                previous.next(next);
            }
        }
    }

    pub fn update(&mut self, index: usize, value: String) {
        match (self.first.as_mut(), index) {
            (None, _) =>
                {}
            (Some(first), 0) =>
                first.value(value),
            (Some(first), index) => {
                first.maybe_at_index_mut(index).map(|node| node.value(value));
            }
        }
    }
}

#[cfg(test)]
mod test_node {
    use crate::linked_list::Node;

    #[test]
    fn should_link_two_nodes() {
        let mut n1 = Node::new("apple".to_string(), None);
        let mut n2 = Node::new("banana".to_string(), None);
        let n3 = Node::new("orange".to_string(), None);
        n2.next(Some(n3));
        assert_eq!(Some(&"orange".to_string()), n2.next.as_ref().map(|n| &n.value));
        n1.next(Some(n2));
        assert_eq!(Some(&"banana".to_string()), n1.next.as_ref().map(|n| &n.value));
    }
}

#[cfg(test)]
mod test_list {
    use crate::linked_list::List;

    #[test]
    fn should_insert() {
        let mut l = List::new();
        l.insert(0, "a".to_string());
        l.insert(1, "b".to_string());
        l.insert(2, "c".to_string());
        l.insert(3, "d".to_string());
        l.insert(4, "e".to_string());
        assert_eq!(Some(&"a".to_string()), l.at(0));
        assert_eq!(Some(&"b".to_string()), l.at(1));
        assert_eq!(Some(&"c".to_string()), l.at(2));
        assert_eq!(Some(&"d".to_string()), l.at(3));
        assert_eq!(Some(&"e".to_string()), l.at(4));
        assert_eq!(None, l.at(5));
        l.insert(0, "aa".to_string()); // insert at 0
        l.insert(4, "bb".to_string()); // insert at tail
        l.insert(100, "cc".to_string()); // index bigger than list length
        assert_eq!(Some(&"aa".to_string()), l.at(0));
        assert_eq!(Some(&"a".to_string()), l.at(1));
        assert_eq!(Some(&"b".to_string()), l.at(2));
        assert_eq!(Some(&"c".to_string()), l.at(3));
        assert_eq!(Some(&"bb".to_string()), l.at(4));
        assert_eq!(Some(&"d".to_string()), l.at(5));
        assert_eq!(Some(&"e".to_string()), l.at(6));
        assert_eq!(Some(&"cc".to_string()), l.at(7));
        assert_eq!(None, l.at(8));
    }

    #[test]
    fn should_find() {
        let mut l = List::new();
        l.insert(0, "a".to_string());
        l.insert(1, "b".to_string());
        l.insert(2, "c".to_string());
        l.insert(3, "d".to_string());
        l.insert(4, "e".to_string());
        assert_eq!(Some(0), l.find(&"a".to_string()));
        assert_eq!(Some(1), l.find(&"b".to_string()));
        assert_eq!(Some(2), l.find(&"c".to_string()));
        assert_eq!(Some(3), l.find(&"d".to_string()));
        assert_eq!(Some(4), l.find(&"e".to_string()));
        assert_eq!(None, l.find(&"f".to_string()));
    }

    #[test]
    fn should_delete() {
        let mut l = List::new();
        l.insert(0, "a".to_string());
        l.insert(1, "b".to_string());
        l.insert(2, "c".to_string());
        l.insert(3, "d".to_string());
        l.insert(4, "e".to_string());
        l.delete(1);
        l.delete(0); // delete at 0
        l.delete(100); // index bigger than list length
        assert_eq!(Some(&"c".to_string()), l.at(0));
        assert_eq!(Some(&"d".to_string()), l.at(1));
        assert_eq!(Some(&"e".to_string()), l.at(2));
        assert_eq!(None, l.at(3));
        l.delete(2); // delete tail
        assert_eq!(None, l.at(2));
    }

    #[test]
    fn should_update() {
        let mut l = List::new();
        l.insert(0, "a".to_string());
        l.insert(1, "b".to_string());
        l.insert(2, "c".to_string());
        l.insert(3, "d".to_string());
        l.insert(4, "e".to_string());
        l.update(0, "aa".to_string()); // update 0
        l.update(1, "bb".to_string());
        l.update(4, "ee".to_string()); // update tail
        l.update(5, "ff".to_string()); // index bigger than list length
        l.update(100, "gg".to_string()); // index bigger than list length
        assert_eq!(Some(&"aa".to_string()), l.at(0));
        assert_eq!(Some(&"bb".to_string()), l.at(1));
        assert_eq!(Some(&"c".to_string()), l.at(2));
        assert_eq!(Some(&"d".to_string()), l.at(3));
        assert_eq!(Some(&"ee".to_string()), l.at(4));
        assert_eq!(None, l.at(5));
    }
}
