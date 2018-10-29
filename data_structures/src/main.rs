pub mod binary_heap;

fn main() {
    init_heap();
}

fn init_heap() {
    use binary_heap::BinaryHeap;

    let mut heap = BinaryHeap::new();
    
    heap.add(2);
    heap.add(12);
    heap.add(32);
    heap.add(23);
    
    BinaryHeap::print_node(&heap.top());
}
