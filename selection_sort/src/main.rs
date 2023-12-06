
#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_out_of_place_selection_sort() {
       let mut array = vec![1, 3, 5, 7, 9];
       assert_eq!(out_of_place_selection_sort(&mut array), [1, 3, 5, 7, 9].to_vec());
   }

   #[test]
   fn test_in_place_selection_sort() {
       let mut array = vec![1, 3, 5, 7, 9];
       in_place_selection_sort(&mut array);
       assert_eq!(array, [1, 3, 5, 7, 9].to_vec());
   }
}

fn find_smallest(array: &Vec<i32>, start: Option<usize>) -> usize {
    let start = start.unwrap_or(0);
    let mut smallest = array[start];
    let mut smallest_index = start;
    for i in (start + 1)..array.len() {
        if array[i] < smallest {
            smallest = array[i];
            smallest_index = i;
        }
    }
    smallest_index
}

fn out_of_place_selection_sort(array: &mut Vec<i32>) -> Vec<i32> {
    let mut sorted = vec![];
    for _ in 0..array.len() {
        let smallest = find_smallest(&array, None);
        sorted.push(array[smallest]);
        array.remove(smallest);
    }
    sorted
}

fn in_place_selection_sort(array: &mut Vec<i32>) {
    for i in 0..array.len() {
        let smallest = find_smallest(&array, Some(i));
        array.swap(i, smallest);
    }
}

fn main() {
    let mut array = vec![9, 7, 5, 3, 1];
    let result = out_of_place_selection_sort(&mut array);
    println!("{:?}", result);

    let mut array = vec![9, 7, 5, 3, 1];
    in_place_selection_sort(&mut array);
    println!("{:?}", array);
}
