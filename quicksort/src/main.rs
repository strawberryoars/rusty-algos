
#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_quicksort() {
       let mut array = vec![1, 3, 5, 7, 9];
       assert_eq!(quicksort(&mut array), [1, 3, 5, 7, 9].to_vec());
   }
}


fn quicksort(array: &mut Vec<i32>) -> Vec<i32> {
    if array.len() < 2 {
        return array.clone();
    }

    let pivot = array[0];
    let mut less = vec![];
    let mut greater = vec![];
    for i in 1..array.len() {
        if array[i] <= pivot {
            less.push(array[i]);
        }
        else {
            greater.push(array[i]);
        }
    }
    let mut left = quicksort(&mut less);
    let mut right = quicksort(&mut greater);
    left.push(pivot);
    left.append(&mut right);
    return left;
}

fn main() {
    let mut array = vec![9, 7, 5, 3, 1];
    let result = quicksort(&mut array);
    println!("{:?}", result);
}
