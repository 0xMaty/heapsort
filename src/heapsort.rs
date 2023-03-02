fn left(i: usize) -> usize {
    (i << 1) + 1
}

fn right(i: usize) -> usize {
    (i << 1) + 2
}

fn max_heapify<T: Ord>(array: &mut [T], i: usize) {
    let l = left(i);
    let r = right(i);
    let mut largest = i;

    if l < array.len() && array[l] > array[i] {
        largest = l;
    }

    if r < array.len() && array[r] > array[largest] {
        largest = r;
    }

    if largest != i {
        array.swap(i, largest);
        max_heapify(array, largest);
    }
}

fn build_max_heap<T: Ord>(array: &mut [T]) {
    for i in (0..array.len() / 2).rev() {
        max_heapify(array, i);
    }
}

// Time complexity: O(NlogN)
pub fn sort<T: Ord>(array: &mut [T]) {
    build_max_heap(array);
    for i in (1..array.len()).rev() {
        array.swap(0, i);
        max_heapify(&mut array[0..i], 0);
    }
}
