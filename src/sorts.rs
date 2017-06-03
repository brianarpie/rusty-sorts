// NOTE: the slightly faster version of this sort was omitted
// to avoid the plethora of casting between usize and i32.
pub fn insertion_sort(array: &mut[i32]) -> &mut[i32] {
  for i in 1..array.len() {
    let mut j = i;
    while j > 0 && array[j-1] > array[j] {
      array.swap(j, j-1);
      j = j - 1;
    }
  }
  array
}

pub fn selection_sort(array: &mut[i32]) -> &mut[i32] {
  let len = array.len();
  let mut min_i;
  for i in 0..len {
    min_i = i;
    for j in (i+1)..len {
      if array[j] < array[min_i] {
        min_i = j;
      }
    }
    array.swap(i, min_i);
  }
  array
}

pub fn merge_sort(array: &mut[i32]) -> &mut[i32] {
  // TODO
  array
}
