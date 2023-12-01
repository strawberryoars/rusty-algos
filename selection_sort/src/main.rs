
// #[cfg(test)]
// mod tests {
//    use super::*;

//    #[test]
//    fn test_selection_sort() {
//        let array = vec![1, 3, 5, 7, 9];
//        assert_eq!(selection_sort(&array), [1, 3, 5, 7, 9].to_vec());
//    }
// }

fn find_smallest(array: &Vec<i32>) -> usize {
    let mut smallest = array[0];
    let mut smallest_index = 0;
    for (index, item) in array.iter().enumerate() {
        if item < &smallest {
            smallest = *item;
            smallest_index = index;
        }
    }

    smallest_index
}

fn selection_sort(array: &mut Vec<i32>) -> Vec<i32> {
    let mut sorted = vec![];
    for _ in 0..array.len() {
        let smallest = find_smallest(&array);
        sorted.push(array[smallest]);
        array.remove(smallest);
    }
    sorted
}

fn main() {
    let mut array = vec![9, 7, 5, 3, 1];
    let result = selection_sort(&mut array);
    println!("{:?}", result);
}
