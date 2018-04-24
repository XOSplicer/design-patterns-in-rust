#![allow(unused)]

struct PrototypeCache<T, F: Fn() -> T> {
    cache: Vec<T>,
    ctor: F,
}

impl<T, F> PrototypeCache<T, F>
    where F: Fn() -> T
{
    fn new(n: usize, ctor: F) -> Self {
        let mut s = PrototypeCache {
            cache: Vec::with_capacity(n),
            ctor
        };
        s.recache(n);
        s
    }
    
    fn recache(&mut self, n: usize) {
        if n > self.cache.len() {
            let needed = n - self.cache.len();
            self.cache.reserve(needed);
            //borrow immutable while self is mutable
            let ctor = &self.ctor; 
            self.cache.extend(
                (0..needed)
                    .map(|_| (ctor)())
            );
        }
    }
    
    fn get(&mut self) -> T {
        self.cache
            .pop()
            .unwrap_or_else(|| (self.ctor)())
    }
    
    fn is_exhausted(&self) -> bool {
        self.cache.is_empty()
    }
}

#[derive(Debug)]
struct A(u32);

fn main() {
    let mut cache = PrototypeCache::new(10, || A(0));
    while ! cache.is_exhausted() {
        println!("{:?}", cache.get());
    }
    println!("exhausted");
    cache.recache(5);
    while ! cache.is_exhausted() {
        println!("{:?}", cache.get());
    }
    println!("exhausted");
}
