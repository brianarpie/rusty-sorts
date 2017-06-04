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

pub fn merge_sort(mut array: &mut[i32]) -> &mut[i32] {
  // TODO: learn how to instantiate the array more idiomatically and dynamically!
  let mut helper:[i32;100000] = [0;100000];
  let low = 0;
  let high = array.len() - 1 ;
  _merge_sort(&mut array, &mut helper, low, high);
  array
}

fn _merge_sort(array: &mut[i32], helper: &mut[i32], low: usize, high: usize) {
  if low < high { 
    let middle = ( low + high ) / 2;
    _merge_sort(array, helper, low, middle);
    _merge_sort(array, helper, middle+1, high);
    merge(array, helper, low, middle, high);
  }
}

fn merge(array: &mut [i32], helper: &mut[i32], low: usize, middle: usize, high: usize) {
  for i in low..high+1 {
    helper[i] = array[i];
  }

  let mut helper_left = low;
  let mut helper_right = middle + 1;
  let mut current = low;

  while helper_left <= middle && helper_right <= high {
    if helper[helper_left] <= helper[helper_right] {
      array[current] = helper[helper_left];
      helper_left += 1;
    } else {
      array[current] = helper[helper_right];
      helper_right += 1;
    }
    current += 1;
  }

  if middle >= helper_left {
    let remaining = middle - helper_left;
    for i in 0..remaining+1 {
      array[current+i] = helper[helper_left+i];
    }
  }

}

