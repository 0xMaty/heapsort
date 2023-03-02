pub mod heapsort;

fn main() {
    // Sort integers.
    let mut integers = [4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
    println!("Unsorted integers: {:?}", integers);
    heapsort::sort(&mut integers);
    println!("Sorted integers: {:?}", integers);

    // Sort strings.
    let mut strings = ["banana", "apple", "orange", "strawberry", "pomegranate"];
    println!("Unsorted strings: {:?}", strings);
    heapsort::sort(&mut strings);
    println!("Sorted strings: {:?}", strings);
}
