#![allow(unused)]

use std::rc::{Rc, Weak};
use std::cell::RefCell;

trait Observable {
    fn attach(&mut self, observer: Rc<Observer<Observed=Self>>);
}

trait Observer {
    type Observed: Observable;
    fn notify(&self, observed: &Self::Observed);
}

struct A {
    observers: Vec<Weak<Observer<Observed=A>>>,
    value: i32
}

impl Observable for A {
    fn attach(&mut self, observer: Rc<Observer<Observed=A>>) {
        self.observers.push(Rc::downgrade(&observer));
    }
}

impl A {
    fn new() -> Self {
        A {
            observers: Vec::new(),
            value: 0
        }
    }

    fn notify_all(&self) {
        self.observers
            .iter()
            .filter_map(|o| o.upgrade())
            .for_each(|o| o.notify(&self));
    }
    
    fn set_value(&mut self, value: i32) {
        self.value = value;
        self.notify_all();
    } 
}

struct AObserver {
    inner: RefCell<i32>
}

impl Observer for AObserver {
    type Observed = A;
    fn notify(&self, observed: &Self::Observed) {
        println!("Notified by observed, value {}.", observed.value);
        *self.inner.borrow_mut() = observed.value;
    }
}

impl AObserver {
    fn new() -> Self {
        AObserver {
            inner: RefCell::new(0)
        }
    }
}

fn main() {
    let mut a = A::new();
    
    let o = Rc::new(AObserver::new());
    a.attach(o.clone());
    a.set_value(1);
    a.set_value(2);
    println!("Observer value: {}", o.inner.borrow());
    println!("Weak observers: {}.", Rc::weak_count(&o));
    
}
