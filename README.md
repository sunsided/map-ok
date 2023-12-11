# MapOk

This crate provides the `MapOk` trait that allows mapping `Ok` variants in an iterator to a different type.

# Examples

```rust
use std::num::ParseIntError;
use std::str::FromStr;
use map_ok::MapOk;

struct Person {
    age: u8,
}

impl Person {
    fn new(age: u8) -> Self {
        Person { age }
    }
}

impl FromStr for Person {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let age = u8::from_str(s)?;
        Ok(Person::new(age))
    }
}

fn example() {
    let input = vec!["10", "20", "x", "30"];
    let mut iterator = input.iter()
        .map(|s| s.parse::<Person>())
        .map_ok(|p| p.age);

    assert_eq!(iterator.next(), Some(Ok(10)));
    assert_eq!(iterator.next(), Some(Ok(20)));
    assert!(iterator.next().unwrap().is_err());
    assert_eq!(iterator.next(), Some(Ok(30)));
    assert_eq!(iterator.next(), None);
}
```