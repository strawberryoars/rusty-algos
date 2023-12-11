
use std::collections::LinkedList;
pub struct HashTable<K, V> {
    elements: Vec<LinkedList<(K, V)>>,
    count: usize,
}

impl<K: Hashable + std::cmp::PartialEq, V> Default for HashTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Hashable {
    fn hash(&self) -> usize;
}

impl<K: Hashable + std::cmp::PartialEq, V> HashTable<K, V> {
    pub fn new() -> HashTable<K, V> {
        let initial_capacity = 3000;
        let mut elements = Vec::with_capacity(initial_capacity);

        for _ in 0..initial_capacity {
            elements.push(LinkedList::new());
        }

        HashTable { elements, count: 0 }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.count >= self.elements.len() * 3 / 4 {
            self.resize();
        }
        let index = key.hash() % self.elements.len();
        self.elements[index].push_back((key, value));
        self.count += 1;
    }

    pub fn search(&self, key: K) -> Option<&V> {
        let index = key.hash() % self.elements.len();
        self.elements[index]
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v)
    }

    fn resize(&mut self) {
        let new_size = self.elements.len() * 2;
        let mut new_elements = Vec::with_capacity(new_size);

        for _ in 0..new_size {
            new_elements.push(LinkedList::new());
        }

        for old_list in self.elements.drain(..) {
            for (key, value) in old_list {
                let new_index = key.hash() % new_size;
                new_elements[new_index].push_back((key, value));
            }
        }

        self.elements = new_elements;
    }
}

#[cfg(test)]
mod tests {
   use super::*;

   struct StringKey {
       key: String,
   }

   impl Hashable for StringKey {
       fn hash(&self) -> usize {
           self.key.len()
       }
   }

   impl PartialEq for StringKey {
       fn eq(&self, other: &Self) -> bool {
           self.key == other.key
       }
   }

   #[test]
   fn test_insert_and_search() {
       let mut ht = HashTable::<StringKey, i32>::new();
       ht.insert(StringKey { key: String::from("hello") }, 1);
       assert_eq!(ht.search(StringKey { key: String::from("hello") }), Some(&1));
       assert_eq!(ht.search(StringKey { key: String::from("world") }), None);
   }

   #[test]
   fn test_resize() {
       let mut ht = HashTable::<StringKey, i32>::new();
       for i in 0..3000 {
           ht.insert(StringKey { key: format!("key{}", i) }, i);
       }
       assert_eq!(ht.elements.len(), 6000);
   }
}

fn main() {

    struct StringKey {
        key: String,
    }
 
    impl Hashable for StringKey {
        fn hash(&self) -> usize {
            self.key.len()
        }
    }
 
    impl PartialEq for StringKey {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key
        }
    }

   let mut ht = HashTable::<StringKey, i32>::new();
   ht.insert(StringKey { key: String::from("hello") }, 1);
   ht.insert(StringKey { key: String::from("world") }, 2);
   println!("{:?}", ht.search(StringKey { key: String::from("hello") })); // prints: Some(1)
   println!("{:?}", ht.search(StringKey { key: String::from("world") })); // prints: Some(2)
   println!("{:?}", ht.search(StringKey { key: String::from("n/a") })); // prints: None
}
