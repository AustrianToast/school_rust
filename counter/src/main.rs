#[derive(Debug)]
struct Counter {
    limit: usize,
    value: usize,
}

impl Counter {
    fn set_limit(&mut self, new_limit: usize) -> usize {
        if new_limit > 0 {
            self.limit = new_limit;
            self.limit
        } else {
            println!("limit needs to be larger than 0");
            self.limit
        }
    }

    fn increment(&mut self) -> usize {
        if self.value < self.limit {
            self.value += 1;
            self.value
        } else {
            self.reset();
            self.value
        }
    }

    fn reset(&mut self) -> usize {
        self.value = 0;
        self.value
    }
}

fn main() {
    let mut counter = Counter {
        limit: 100,
        value: 98,
    };

    println!("{:?}", counter);

    println!("incremented to {}", counter.increment());

    println!("incremented to {}", counter.increment());

    println!("overflowed to {}", counter.increment());

    println!("new limit is {}", counter.set_limit(10));

    println!("incremented to {}", counter.increment());
}
