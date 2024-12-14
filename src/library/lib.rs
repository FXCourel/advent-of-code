#![allow(dead_code)]

pub mod binary_heap_node {
    pub struct BinaryHeapNode<T> {
        pub value: T,
        pub priority: i32,
    }

    impl<T> BinaryHeapNode<T> {
        pub fn new(value: T, priority: i32) -> Self {
            Self { value, priority }
        }
    }

    impl<T> PartialEq for BinaryHeapNode<T> {
        fn eq(&self, other: &Self) -> bool {
            self.priority == other.priority
        }
    }

    impl<T> Eq for BinaryHeapNode<T> {}

    impl<T> PartialOrd for BinaryHeapNode<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<T> Ord for BinaryHeapNode<T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.priority.cmp(&other.priority)
        }
    }
}
