

fn binary_search(array: &[i32], item: i32) -> Option<usize> {
    let low = 0;
    let mut high = array.len();

    while low <= high {
        let mid = (low + high) / 2;
        let guess = array[mid];
        
        if guess == item {
            mid;
        }

        if guess > item {
            high = mid - 1;
        }

        if guess < item {
            high = mid + 1;
        }

    }

    None
}


fn main() {
    println!("Hello, world!");
    let array = [1, 3, 5, 7, 9];
    let result = binary_search(&array, 3);
    println!("{}", result);
}
