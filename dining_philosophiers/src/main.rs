use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Philosophier {
    name: String,
    left: usize,
    right: usize,

}

impl Philosophier {
    fn new(name: &str, left: usize, right: usize) -> Philosophier {
        Philosophier {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophiers = vec![
        Philosophier::new("Judith Butler", 0, 1),
        Philosophier::new("Gilles Deleuze", 1, 2),
        Philosophier::new("Karl Marx", 2, 3),
        Philosophier::new("Emma Goldman", 3, 4),
        Philosophier::new("Michel Foucault", 0, 4),
    ];

    let handles: Vec<_> = philosophiers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
