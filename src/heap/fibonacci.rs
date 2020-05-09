use std::collections::LinkedList;

#[derive(Debug)]
pub struct InternalTree<T: std::cmp::Ord> {
    degree: usize,
    payload: Option<T>,
    children_list: LinkedList<InternalTree<T>>,
}

impl<T: std::cmp::Ord> InternalTree<T> {
    pub fn init(payload: T) -> InternalTree<T> {
        InternalTree {
            degree: 0,
            payload: Some(payload),
            children_list: LinkedList::new(),
        }
    }

    pub fn is_smaller_or_equal(
        internal_tree_1: &InternalTree<T>,
        internal_tree_2: &InternalTree<T>,
    ) -> bool {
        match (
            internal_tree_1.peek_payload(),
            internal_tree_2.peek_payload(),
        ) {
            (Some(payload1), Some(payload2)) => payload1 <= payload2,
            _ => panic!("Payloads can not be empty"),
        }
    }

    pub fn merge(
        mut internal_tree_1: InternalTree<T>,
        mut internal_tree_2: InternalTree<T>,
    ) -> InternalTree<T> {
        if InternalTree::is_smaller_or_equal(&internal_tree_1, &internal_tree_2) {
            internal_tree_1.add_child(internal_tree_2);

            internal_tree_1
        } else {
            internal_tree_2.add_child(internal_tree_1);

            internal_tree_2
        }
    }

    fn add_child(&mut self, internal_tree: InternalTree<T>) {
        self.children_list.push_back(internal_tree);
        self.degree += 1;
    }

    pub fn degree(&self) -> usize {
        self.degree
    }

    pub fn peek_payload(&self) -> &Option<T> {
        &self.payload
    }

    pub fn get_payload(&mut self) -> T {
        if self.payload.is_none() {
            panic!("Payload is None");
        }

        self.payload.take().unwrap()
    }

    pub fn children_list_mut(&mut self) -> &mut LinkedList<InternalTree<T>> {
        &mut self.children_list
    }

    pub fn children_list(&self) -> &LinkedList<InternalTree<T>> {
        &self.children_list
    }
}

impl<T> InternalTree<T>
where
    T: std::cmp::Ord + std::fmt::Display,
{
    pub fn preorder(internal_tree: &InternalTree<T>) -> String {
        return String::from(InternalTree::_preorder(&Some(internal_tree)).trim());
    }

    fn _preorder(node_opt: &Option<&InternalTree<T>>) -> String {
        let mut node_list = String::from("");

        match node_opt {
            None => node_list,
            Some(node) => {
                match node.peek_payload() {
                    Some(value) => node_list.push_str(format!("{} ", value).as_str()),
                    None => (),
                }
                for item in node.children_list() {
                    node_list
                        .push_str(format!("{}", InternalTree::_preorder(&Some(&item))).as_str());
                }
                node_list
            }
        }
    }
}

#[cfg(test)]
mod internal_tree_tests {
    use super::*;

    #[test]
    fn heap_fibonacci_internal_tree_init() {
        let it = InternalTree::init(1);

        assert_eq!(it.degree(), 0);
        assert_eq!(*it.peek_payload(), Some(1));
    }

    #[test]
    fn heap_fibonacci_internal_tree_is_smaller() {
        let it1 = InternalTree::init(0);
        let it2 = InternalTree::init(1);
        let it3 = InternalTree::init(0);

        assert_eq!(InternalTree::is_smaller_or_equal(&it1, &it2), true);
        assert_eq!(InternalTree::is_smaller_or_equal(&it1, &it3), true);
        assert_eq!(InternalTree::is_smaller_or_equal(&it2, &it1), false);
    }

    #[test]
    fn heap_fibonacci_internal_tree_add_child_1() {
        let mut it1 = InternalTree::init(0);
        let it2 = InternalTree::init(1);

        it1.add_child(it2);

        assert_eq!(it1.degree(), 1);
        assert_eq!(
            *it1.children_list.pop_back().unwrap().peek_payload(),
            Some(1)
        );
    }

    #[test]
    fn heap_fibonacci_internal_tree_add_child_2() {
        let it1 = InternalTree::init(0);
        let mut it2 = InternalTree::init(1);

        it2.add_child(it1);

        assert_eq!(it2.degree(), 1);
        assert_eq!(
            *it2.children_list.pop_back().unwrap().peek_payload(),
            Some(0)
        );
    }

    #[test]
    fn heap_fibonacci_internal_tree_merge_1() {
        let it1 = InternalTree::init(0);
        let it2 = InternalTree::init(1);

        let mut merged_tree = InternalTree::merge(it1, it2);

        assert_eq!(merged_tree.degree(), 1);
        assert_eq!(
            *merged_tree.children_list.pop_back().unwrap().peek_payload(),
            Some(1)
        );
    }

    #[test]
    fn heap_fibonacci_internal_tree_merge_2() {
        let it1 = InternalTree::init(0);
        let it2 = InternalTree::init(1);

        let mut merged_tree = InternalTree::merge(it2, it1);

        assert_eq!(merged_tree.degree(), 1);
        assert_eq!(
            *merged_tree.children_list.pop_back().unwrap().peek_payload(),
            Some(1)
        );
    }

    #[test]
    fn heap_fibonacci_internal_tree_merge_3() {
        let it1 = InternalTree::init(0);
        let it2 = InternalTree::init(1);
        let merged_tree_1 = InternalTree::merge(it2, it1);
        let it3 = InternalTree::init(2);
        let it4 = InternalTree::init(3);
        let merged_tree_2 = InternalTree::merge(it3, it4);

        let merged_tree = InternalTree::merge(merged_tree_1, merged_tree_2);

        assert_eq!(merged_tree.degree(), 2);
        assert_eq!(
            InternalTree::preorder(&merged_tree),
            String::from("0 1 2 3")
        );
    }
}

// ------------- Fibonacci Heap -------------
#[derive(Debug)]
pub struct FibonacciHeap<T: std::cmp::Ord> {
    children_list: LinkedList<InternalTree<T>>,
    size: usize,
    min_pointer: Option<InternalTree<T>>,
}

impl<T: std::cmp::Ord> FibonacciHeap<T> {
    pub fn init() -> FibonacciHeap<T> {
        FibonacciHeap {
            children_list: LinkedList::new(),
            size: 0,
            min_pointer: None,
        }
    }

    pub fn push(&mut self, payload: T) {
        let new_node = InternalTree::init(payload);

        if self.min_pointer.is_none() {
            self.min_pointer = Some(new_node);
        } else {
            if InternalTree::is_smaller_or_equal(&new_node, &self.min_pointer.as_ref().unwrap()) {
                let temp = self.min_pointer.take().unwrap();
                self.min_pointer = Some(new_node);
                self.children_list.push_back(temp);
            } else {
                self.children_list.push_back(new_node);
            }
        }

        self.size += 1;
    }

    pub fn merge(
        mut fibonacci_heap_1: FibonacciHeap<T>,
        mut fibonacci_heap_2: FibonacciHeap<T>,
    ) -> FibonacciHeap<T> {
        fibonacci_heap_1
            .children_list
            .append(&mut fibonacci_heap_2.children_list);

        if InternalTree::is_smaller_or_equal(
            &fibonacci_heap_2.min_pointer.as_ref().unwrap(),
            &fibonacci_heap_1.min_pointer.as_ref().unwrap(),
        ) {
            let temp = fibonacci_heap_1.min_pointer.take().unwrap();
            fibonacci_heap_1.min_pointer = fibonacci_heap_2.min_pointer.take();
            fibonacci_heap_1.children_list.push_back(temp);

            fibonacci_heap_1.size += fibonacci_heap_2.size;
        } else {
            fibonacci_heap_1.push(fibonacci_heap_2.min_pointer.unwrap().get_payload());

            fibonacci_heap_1.size += fibonacci_heap_2.size - 1;
        }

        fibonacci_heap_1
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let mut min_node = self.min_pointer.take().unwrap();

        self.size -= 1;
        let mut next = min_node.children_list.pop_front();

        while !next.is_none() {
            let child = next.unwrap();

            self.children_list.push_back(child);

            next = min_node.children_list.pop_front();
        }

        let payload = min_node.get_payload();

        if !self.is_empty() {
            self.min_pointer = self.children_list.pop_front();

            self.consolidate();
        }

        Some(payload)
    }

    fn consolidate(&mut self) {
        // there is nothing to consolidate
        if self.is_empty() {
            return;
        }

        // array size will be log(heap size) with base 1.61803
        let array_size = ((self.size as f32).log(1.61803_f32) + 1.0) as usize;

        let mut a: Vec<Option<InternalTree<T>>> = Vec::with_capacity(array_size);

        // initialize consolidate array
        for _ in 0..array_size {
            a.push(None);
        }

        // add min node to children list
        self.children_list
            .push_front(self.min_pointer.take().unwrap());

        let mut next = self.children_list.pop_front();

        while !next.is_none() {
            let mut x = next.unwrap();
            let mut d = x.degree();
            while !a[d].is_none() {
                let y = a[d].take().unwrap();
                x = InternalTree::merge(x, y);
                d += 1;
            }
            a[d] = Some(x);

            next = self.children_list.pop_front();
        }

        // update min pointer and children list
        self.min_pointer = None;
        for i in 0..array_size {
            if !a[i].is_none() {
                if self.min_pointer.is_none() {
                    self.min_pointer = a[i].take();
                } else {
                    if InternalTree::is_smaller_or_equal(
                        &a[i].as_ref().unwrap(),
                        &self.min_pointer.as_ref().unwrap(),
                    ) {
                        let temp = self.min_pointer.take().unwrap();
                        self.min_pointer = a[i].take();
                        self.children_list.push_back(temp);
                    } else {
                        self.children_list.push_back(a[i].take().unwrap());
                    }
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl<T> FibonacciHeap<T>
where
    T: std::cmp::Ord + std::fmt::Display,
{
    pub fn preorder(fibonacci_heap: &FibonacciHeap<T>) -> String {
        let mut node_list = String::from("");

        if !fibonacci_heap.min_pointer.is_none() {
            node_list.push_str(
                format!(
                    "Min: {}\n",
                    InternalTree::preorder(&fibonacci_heap.min_pointer.as_ref().unwrap())
                )
                .as_str(),
            );
        }

        for (index, internal_tree) in fibonacci_heap.children_list.iter().enumerate() {
            node_list.push_str(format!("Tree {}: ", index + 1).as_str());
            node_list.push_str(InternalTree::preorder(&internal_tree).as_str());

            node_list.push_str("\n");
        }

        node_list
    }
}

#[cfg(test)]
mod fibonacci_heap_tests {
    use super::*;

    #[test]
    fn heap_fibonacci_init() {
        let fh: FibonacciHeap<usize> = FibonacciHeap::init();

        assert!(fh.min_pointer.is_none());
        assert_eq!(fh.size, 0);
        assert_eq!(fh.children_list.len(), 0);
    }

    #[test]
    fn heap_fibonacci_push_1() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init();

        fh.push(0);
        fh.push(1);
        fh.push(3);

        assert_eq!(fh.children_list.len(), 2);
        assert_eq!(fh.min_pointer.as_ref().unwrap().peek_payload().unwrap(), 0);

        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Min: 0\nTree 1: 1\nTree 2: 3\n")
        )
    }

    #[test]
    fn heap_fibonacci_push_2() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init();

        fh.push(3);
        fh.push(1);
        fh.push(0);

        assert_eq!(fh.children_list.len(), 2);
        assert_eq!(fh.min_pointer.as_ref().unwrap().peek_payload().unwrap(), 0);

        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Min: 0\nTree 1: 3\nTree 2: 1\n")
        )
    }

    #[test]
    fn heap_fibonacci_merge_1() {
        let mut fh1: FibonacciHeap<usize> = FibonacciHeap::init();
        fh1.push(0);

        let mut fh2: FibonacciHeap<usize> = FibonacciHeap::init();
        fh2.push(1);

        let merged_heap = FibonacciHeap::merge(fh1, fh2);

        assert_eq!(merged_heap.size, 2);
        assert_eq!(
            merged_heap
                .min_pointer
                .as_ref()
                .unwrap()
                .peek_payload()
                .unwrap(),
            0
        );
        assert_eq!(
            FibonacciHeap::preorder(&merged_heap),
            String::from("Min: 0\nTree 1: 1\n")
        );
    }

    #[test]
    fn heap_fibonacci_merge_2() {
        let mut fh1: FibonacciHeap<usize> = FibonacciHeap::init();
        fh1.push(0);
        fh1.push(2);

        let mut fh2: FibonacciHeap<usize> = FibonacciHeap::init();
        fh2.push(1);
        fh2.push(3);

        let merged_heap = FibonacciHeap::merge(fh2, fh1);

        assert_eq!(merged_heap.size, 4);
        assert_eq!(
            merged_heap
                .min_pointer
                .as_ref()
                .unwrap()
                .peek_payload()
                .unwrap(),
            0
        );
        assert_eq!(
            FibonacciHeap::preorder(&merged_heap),
            String::from("Min: 0\nTree 1: 3\nTree 2: 2\nTree 3: 1\n")
        );
    }

    #[test]
    fn heap_fibonacci_consolidate_1() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init();
        fh.push(0);

        fh.consolidate();

        assert_eq!(FibonacciHeap::preorder(&fh), String::from("Min: 0\n"));
    }

    #[test]
    fn heap_fibonacci_consolidate_2() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init();
        fh.push(1);
        fh.push(0);

        fh.consolidate();

        assert_eq!(FibonacciHeap::preorder(&fh), String::from("Min: 0 1\n"));
    }

    #[test]
    fn heap_fibonacci_consolidate_3() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init();
        fh.push(1);
        fh.push(0);
        fh.push(2);

        fh.consolidate();

        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Min: 0 1\nTree 1: 2\n")
        );
    }

    #[test]
    fn heap_fibonacci_consolidate_4() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init();
        fh.push(1);
        fh.push(0);
        fh.push(2);
        fh.push(3);

        fh.consolidate();

        assert_eq!(FibonacciHeap::preorder(&fh), String::from("Min: 0 1 2 3\n"));
    }

    #[test]
    fn heap_fibonacci_consolidate_5() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init();
        fh.push(1);
        fh.push(0);
        fh.push(3);
        fh.push(2);
        fh.push(4);

        fh.consolidate();

        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Min: 0 1 2 3\nTree 1: 4\n")
        );
    }

    #[test]
    fn heap_fibonacci_consolidate_6() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init();
        for i in 0..14 {
            fh.push(i)
        }

        fh.consolidate();

        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Min: 0 1 2 3 4 5 6 7\nTree 1: 12 13\nTree 2: 8 9 10 11\n")
        );
    }

    #[test]
    fn heap_fibonacci_pop_1() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init();
        fh.push(0);

        assert_eq!(fh.pop(), Some(0));

        assert_eq!(FibonacciHeap::preorder(&fh), String::from(""));
    }

    #[test]
    fn heap_fibonacci_pop_2() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init();
        fh.push(0);
        fh.push(1);

        assert_eq!(fh.pop(), Some(0));
        assert_eq!(fh.size(), 1);
        assert_eq!(FibonacciHeap::preorder(&fh), String::from("Min: 1\n"));
    }

    #[test]
    fn heap_fibonacci_pop_3() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init();
        fh.push(2);
        fh.push(0);
        fh.push(1);

        assert_eq!(fh.pop(), Some(0));
        assert_eq!(fh.size(), 2);
        assert_eq!(FibonacciHeap::preorder(&fh), String::from("Min: 1 2\n"));
    }

    #[test]
    fn heap_fibonacci_pop_4() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init();
        fh.push(2);
        fh.push(3);
        fh.push(0);
        fh.push(1);

        assert_eq!(fh.pop(), Some(0));
        assert_eq!(fh.size(), 3);
        assert_eq!(FibonacciHeap::preorder(&fh), String::from("Min: 1\nTree 1: 2 3\n"));
    }

    #[test]
    fn heap_fibonacci_pop_multi_1() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init();
        fh.push(2);
        fh.push(3);
        fh.push(0);
        fh.push(1);

        assert_eq!(fh.pop(), Some(0));
        assert_eq!(fh.size(), 3);

        assert_eq!(fh.pop(), Some(1));
        assert_eq!(fh.size(), 2);
        assert_eq!(FibonacciHeap::preorder(&fh), String::from("Min: 2 3\n"));

        assert_eq!(fh.pop(), Some(2));
        assert_eq!(fh.size(), 1);
        assert_eq!(FibonacciHeap::preorder(&fh), String::from("Min: 3\n"));

        assert_eq!(fh.pop(), Some(3));
        assert_eq!(fh.size(), 0);
        assert_eq!(FibonacciHeap::preorder(&fh), String::from(""));

        assert_eq!(fh.pop(), None);
        assert_eq!(fh.size(), 0);
        assert_eq!(FibonacciHeap::preorder(&fh), String::from(""));
    }

    #[test]
    fn heap_fibonacci_pop_multi_2() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::init();
        for i in 0..5 {
            fh.push(i)
        }
        
        assert_eq!(fh.pop(), Some(0));
        assert_eq!(fh.size(), 4);
        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Min: 1 2 3 4\n")
        );

        assert_eq!(fh.pop(), Some(1));
        assert_eq!(fh.size(), 3);
        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Min: 2\nTree 1: 3 4\n")
        );

        assert_eq!(fh.pop(), Some(2));
        assert_eq!(fh.size(), 2);
        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Min: 3 4\n")
        );

        assert_eq!(fh.pop(), Some(3));
        assert_eq!(fh.size(), 1);
        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("Min: 4\n")
        );

        assert_eq!(fh.pop(), Some(4));
        assert_eq!(fh.size(), 0);
        assert_eq!(
            FibonacciHeap::preorder(&fh),
            String::from("")
        );
    }


}
