pub mod binary_heap;
pub mod sort_algorithms;
pub mod tests;
fn main() {
    test_binary_heap();
    test_heap_sort()
}

fn test_binary_heap() {
    use binary_heap::BinaryHeap;

    let mut heap = BinaryHeap::new();
    
    heap.add(2);
    heap.add(12);
    heap.add(32);
    heap.add(23);
    
    BinaryHeap::print_node(&heap.top());
}

fn test_heap_sort() {
    use sort_algorithms::*;

    let mut arr = Vec::new();

    arr.push(525);
    arr.push(2121);
    arr.push(32);
    arr.push(30);
    arr.push(5325);
    arr.push(5252);
    arr.push(4);
    arr.push(4);
    arr.push(535);

    heap_sort(&mut arr);

    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }
}


