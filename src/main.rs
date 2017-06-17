mod sorts;
mod benchmark;
extern crate rand;
use rand::distributions::{IndependentSample, Range};
use std::i32;

fn main() {
    // TODO: move the array instantiation into a separate method.
    const ARRAY_LENGTH: usize = 10000;
    let mut array: [i32; ARRAY_LENGTH] = [0;ARRAY_LENGTH];

    let between = Range::new(0, 1000);
    let mut rng = rand::thread_rng();

    for i in 0..ARRAY_LENGTH {
        array[i] = between.ind_sample(&mut rng);
    }

    let mut array1 = array;
    let insertion = sorts::insertion_sort;
    let insertion_time = time_sort(&insertion, &mut array1);

    let mut array2 = array;
    let selection = sorts::selection_sort;
    let selection_time = time_sort(&selection, &mut array2);

    let mut array3 = array;
    let merge = sorts::merge_sort;
    let merge_time = time_sort(&merge, &mut array3);

    let mut array4 = array;
    let quick = sorts::quick_sort;
    let quick_time = time_sort(&quick, &mut array4);

    println!("sorting {} elements", ARRAY_LENGTH);
    println!("selection: {}ms", selection_time);
    println!("insertion: {}ms", insertion_time);
    println!("merge: {}ms", merge_time);
    println!("quick: {}ms", quick_time);
}

fn time_sort(sort: &Fn(&mut[i32]) -> &mut[i32], array: &mut[i32]) -> usize {
    let mut timer = benchmark::Timer::new();

    timer.start();
    let sorted_array = sort(array);
    timer.stop();

    verify_sorted(sorted_array);

    timer.get_elapsed_in_ms() as usize
}

fn verify_sorted(array: &mut[i32]) {
    let mut i_min = i32::MIN;
    for i in 0..array.len() {
        if array[i] >= i_min {
            i_min = array[i];
        } else {
            panic!("Sort method failed to sort!");
        }
    }
}
