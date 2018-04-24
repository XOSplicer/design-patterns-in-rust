use std::sync::{Arc, Mutex};

trait Command {
    type E;
    fn execute(&self) -> Result<(), Self::E>;
    fn undo(&self) -> Result<(), Self::E>;
}

struct Reciever {
    count: u32
}

impl Reciever {
    fn new() -> Self {
        Reciever {
            count: 0
        }
    }
    fn increment(&mut self) -> Result<(), String> {
        let prev = self.count;
        self.count = self.count.saturating_add(1);
        if prev==self.count {
            return Err("Counter overflow".into());
        }
        Ok(())
    }
    fn decrement(&mut self) -> Result<(), String> {
        let prev = self.count;
        self.count = self.count.saturating_sub(1);
        if prev==self.count {
            return Err("Counter underflow".into());
        }
        Ok(())
    }
}

struct IncreaseCommand {
    reciever: Arc<Mutex<Reciever>>
}

impl Command for IncreaseCommand {
    type E = String;
    fn execute(&self) -> Result<(), Self::E> {
        let mut inner = self.reciever.lock().unwrap();
        inner.increment()
    }
    fn undo(&self) -> Result<(), Self::E> {
        let mut inner = self.reciever.lock().unwrap();
        inner.decrement()
    }
}

struct Invoker {
    commands: Vec<Box<Command<E=String>>>,
    next: usize,
}

impl Invoker {
    fn new() -> Self {
        Invoker {
            commands: Vec::new(),
            next: 0
        }
    }
    fn push_command(&mut self, c: Box<Command<E=String>>) {
        self.commands.push(c);
    }
    fn do_next(&mut self) -> Result<(), String> {
        if self.next >= self.commands.len() {
            return Err("No next command available".into());
        }
        self.next += 1;
        self.commands[self.next-1].execute()
    }
    fn undo_prev(&mut self) -> Result<(), String> {
        if self.next == 0 {
            return Err("No previous command available".into());
        }
        self.next -= 1;
        self.commands[self.next].undo()
    }
}

fn main() {
    let counterA = Arc::new(Mutex::new(Reciever::new()));
    let counterB = Arc::new(Mutex::new(Reciever::new()));
    let mut invoker = Invoker::new();
    invoker.push_command(Box::new(IncreaseCommand{reciever: counterA.clone()}));
    invoker.push_command(Box::new(IncreaseCommand{reciever: counterA.clone()}));
    invoker.push_command(Box::new(IncreaseCommand{reciever: counterB.clone()}));
    invoker.push_command(Box::new(IncreaseCommand{reciever: counterA.clone()}));
    invoker.push_command(Box::new(IncreaseCommand{reciever: counterB.clone()}));
    println!("A: {}", counterA.lock().unwrap().count);
    println!("B: {}", counterB.lock().unwrap().count);
    for _ in 0..invoker.commands.len() {
        invoker.do_next().unwrap();
        println!("A: {}", counterA.lock().unwrap().count);
        println!("B: {}", counterB.lock().unwrap().count);
    }
    for _ in 0..invoker.commands.len() {
        invoker.undo_prev().unwrap();
        println!("A: {}", counterA.lock().unwrap().count);
        println!("B: {}", counterB.lock().unwrap().count);
    }
}
