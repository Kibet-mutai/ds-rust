use std::collections::BinaryHeap;


fn hep(a: &[i32]) -> BinaryHeap<i32> {
    let mut heap = BinaryHeap::new();
    for &i in a {

    heap.push(i);
    }
     heap
}
fn cookies(k: i32, a: &[i32]) -> i32 {
   let mut heap = hep(a);

    let mut operations = 0;

    while heap.len() >= 2 {
        let least_sweet = heap.pop().unwrap();
        let second_least_sweet = heap.pop().unwrap();

        if least_sweet >= k {
            return operations;
        }

        let new_cookie_sweetness = least_sweet + 2 * second_least_sweet;
        heap.push(new_cookie_sweetness);

        operations += 1;
    }

    if let Some(sweetness) = heap.pop() {
        if sweetness >= k {
            return operations;
        }
    }

    -1
}
fn main() {
    unimplemented!();
}
