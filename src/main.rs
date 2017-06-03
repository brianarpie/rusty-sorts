mod sorts;

fn main() {
    let arr = [5, 7, 1, 9, 4, 12, 3, 2, 1, 0, 11];

    let mut arr1 = arr;
    let insertion = sorts::insertion_sort(&mut arr1);

    let mut arr2 = arr;
    let selection = sorts::selection_sort(&mut arr2);

    let mut arr3 = arr;
    let merge = sorts::merge_sort(&mut arr3);

    println!("insertion sort: {:?}", insertion);
    println!("selection sort: {:?}", selection);
    println!("merge sort: {:?}", merge);
}
