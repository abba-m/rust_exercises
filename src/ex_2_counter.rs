use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// Counter counts the number of times each value of type T has been seen.
#[derive(Debug)]
struct Counter<T> {
    values: HashMap<T, u64>,
}

impl<T: Debug> PartialEq for Counter<T> {
  fn eq(&self, other: &Self) -> bool {
      // TODO: implement eq
      false
  }
}

impl<T: Debug  + Eq + PartialEq + Hash> Counter<T> {
    /// Create a new Counter.
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// Count an occurrence of the given value.
    fn count(&mut self, value: T) {
        // *self.values.entry(value).or_insert(0) += 1;
      self.values.entry(value)
        .and_modify(|count| *count += 1)
        .or_insert(1);
    }

    /// Return the number of times the given value has been seen.
    fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}

pub fn counter() {
    let mut ctr = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);

    for i in 10..20 {
        println!("saw {} values equal to {}", ctr.times_seen(i), i);
    }

    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    println!("got {} apples", strctr.times_seen("apple"));


    let mut strctr2: Counter<&str> = Counter::new();
    strctr2.count("car");
    strctr2.count("car");
    strctr2.count("bicycle");
    strctr2.count("house");
    strctr2.count("house");
    strctr2.count("house");
      


    println!("Are values equal? {:?}", strctr2 == strctr)
}
