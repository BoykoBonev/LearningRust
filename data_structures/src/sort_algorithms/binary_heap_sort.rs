pub fn heap_sort<T: Ord>(elements: &mut Vec<T>) {
    let elements_count = elements.len();
    for index in (0..elements.len() / 2).rev() {
        heapify_down(elements, index, elements_count);
    }

    for index in (0..elements.len()).rev() {
        elements.swap(index, 0);
        heapify_down(elements, 0, index);
    }

}

pub fn heapify_down<T: Ord>(elements: &mut Vec<T>, mut index: usize, count: usize) {
    while (index * 2) + 2 < count {
        let bigger_child_index;
        if elements[(index * 2) + 1] > elements[(index * 2) + 2] {
            bigger_child_index = (index * 2) + 1;
        } else {
            bigger_child_index = (index * 2) + 2;
        }

        if elements[bigger_child_index] > elements[index] {
            elements.swap(bigger_child_index, index);
            index = bigger_child_index;

        } else {
            break;
        }
    }

    if index * 2 + 1 < count && elements[index * 2 + 1] > elements[index] {
        elements.swap((index * 2) + 1, index);
    }
}