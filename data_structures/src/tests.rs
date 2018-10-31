#[cfg(test)]
mod test {
    use binary_heap::BinaryHeap;

    #[test]
    fn binary_heap_count_test() {
        let mut heap = BinaryHeap::new();
        let expected_elements_count = 10;

        for i in 0..expected_elements_count  {
            heap.add(i);
        }
        assert_eq!(heap.count(), expected_elements_count);
    }

    #[test]
    fn binary_heap_add_test() {
        // let mut heap = BinaryHeap::new();

        // add couple of elements

        // assert that heapify_up is called the right number of times        
    }
}