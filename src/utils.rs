use crate::base_types::Intersection;

pub const FLOAT_DIFF: f64 = 0.00001;

pub fn float_equal(i: f64, j: f64) -> bool {
    let diff = i - j;
    diff < FLOAT_DIFF && diff > -FLOAT_DIFF
}

// merge portion of merge sort, takes in one array that has two sorted
// arrays in it. separator is the index where the first sorted array ends
pub fn combine_sorted(vec: &mut [Intersection], separator: usize) {
    let len = vec.len();
    if len > 1 {
        merge(vec, 0, separator, len - 1);
    }
}

fn merge(vec: &mut [Intersection], l: usize, mid: usize, r: usize) {
    let mut temp_vec = Vec::with_capacity(r - l + 1);
    let mut left_index = l;
    let mut right_index = mid;

    while left_index < mid && right_index <= r {
        if vec[left_index].distance <= vec[right_index].distance {
            temp_vec.push(vec[left_index].clone());
            left_index += 1;
        } else {
            temp_vec.push(vec[right_index].clone());
            right_index += 1;
        }
    }

    while left_index < mid {
        temp_vec.push(vec[left_index].clone());
        left_index += 1;
    }

    while right_index <= r {
        temp_vec.push(vec[right_index].clone());
        right_index += 1;
    }

    left_index = l;
    for sorted_val in temp_vec.into_iter() {
        vec[left_index] = sorted_val;
        left_index += 1;
    }
}

pub fn is_sorted(vec: &Vec<Intersection>) -> bool {
    let len = vec.len();
    for i in 1..len {
        if vec[i].distance < vec[i - 1].distance {
            return false;
        }
    }

    true
}
