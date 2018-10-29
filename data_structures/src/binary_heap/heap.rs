use std;

pub struct BinaryHeap<T> {
    heap: Vec<T>
}

impl<T: Ord> BinaryHeap<T> where T: std::fmt::Display {
    pub fn new() -> BinaryHeap<T> {
        let heap = Vec::new();
        BinaryHeap { heap: heap }
    }

     pub fn print_node(node: &Option<&T>) {
        match node {
            Some(value) => {
                println!("{} ", value)
            },
            None => {
                println!("Empty")
            }
        }
    }

    pub fn top(&self) -> Option<&T> {
        if self.heap.len() > 0 {
            Some(&self.heap[0])
        } else {
            None
        }
    }

    pub fn count(&self) -> usize {
        self.heap.len()
    }

    pub fn add(&mut self, element: T) {   
        let mut index = self.count();
        self.heap.push(element);

        while (index > 0) && (self.heap[index] > self.heap[index / 2]) {
            self.heap.swap(index, (index - 1) / 2);
            index = (index - 1 ) / 2;
        }
    }

    pub fn remove_top(&mut self) -> T{
        let top_element = self.heap.swap_remove(0);
        let mut index = 0;
        while (index * 2) + 2 <= self.count() {
         
            let bigger_child_index;
            if self.heap[(index * 2) + 1] > self.heap[(index * 2) + 2] {
                bigger_child_index = (index * 2) + 1;
            } else {
                bigger_child_index = (index * 2) + 2;
            }

            if self.heap[bigger_child_index] > self.heap[index] {
                self.heap.swap(bigger_child_index, index);
                index = bigger_child_index;
            } else {
                break;
            }
        }

        if (index * 2) + 1 < self.count() && self.heap[(index * 2) + 1] > self.heap[index] {
            self.heap.swap((index * 2) + 1, index);
        }

        top_element
    }
}