
#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_binary_search() {
       let array = [1, 3, 5, 7, 9];
       assert_eq!(binary_search(&array, 1), Some(0));
       assert_eq!(binary_search(&array, 3), Some(1));
       assert_eq!(binary_search(&array, 5), Some(2));
       assert_eq!(binary_search(&array, 7), Some(3));
       assert_eq!(binary_search(&array, 9), Some(4));
       assert_eq!(binary_search(&array, 11), None);
   }
}

fn binary_search(array: &[i32], item: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = array.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let guess = array[mid];
        
        if guess == item {
            return Some(mid);
        }

        if guess > item {
            high = mid - 1;
        }

        if guess < item {
            low = mid + 1;
        }
    }

    None
}

fn main() {
    let array = [1, 3, 5, 7, 9];
    let result = binary_search(&array, 3);
    println!("{:?}", result);
}
