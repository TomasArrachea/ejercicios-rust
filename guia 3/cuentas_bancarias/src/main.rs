use std::thread;
use std::sync::{Arc, Mutex};

struct Account(i32);

impl Account {
    fn deposit(&mut self, amount: i32) {
        println!("op: deposit {}, available funds: {:?}", amount, self.0);
        self.0 += amount;
    }
    
    fn withdraw(&mut self, amount: i32) {
        println!("op: withdraw {}, available funds: {}", amount, self.0);
        if self.0 >= amount {
            self.0 -= amount;
        } else {
            panic!("Error: Insufficient funds.")
        }
    }
    
    fn balance(&self) -> i32 {
        self.0
    }
}

// static mut ACCOUNT: Account = Account(0);

fn main() {
    let counter = Arc::new(Mutex::new(Account(0))); //tomar referencias mutables de una variable global es unsafe.

    let counter_clone = Arc::clone(&counter);
    let customer1_handle = thread::spawn(move || {
        counter_clone.lock().unwrap().deposit(40);
    });
    customer1_handle.join().unwrap(); // !

    let counter_clone = Arc::clone(&counter);
    let customer2_handle = thread::spawn(move || {
        counter_clone.lock().unwrap().withdraw(30);
    });
    customer2_handle.join().unwrap();// !

    let counter_clone = Arc::clone(&counter);
    let customer3_handle = thread::spawn(move || {
        counter_clone.lock().unwrap().deposit(60);
    });
    customer3_handle.join().unwrap(); // !
    
    let counter_clone = Arc::clone(&counter);
    let customer4_handle = thread::spawn(move || {
        counter_clone.lock().unwrap().withdraw(70);
    });
    customer4_handle.join().unwrap(); // !

    // COMO HAGO PARA EVITAR CONDICIONES DE CARRERA?
    // SI NO SE PUEDEN EJECUTAR LAS ACCIONES EN CUALQUIER ORDEN NO TIENE SENTIDO USAR CONCURRENCIA.
    
    // let handles = vec![
    // // customer1_handle,
    // // customer2_handle,
    // // customer3_handle,
    // // customer4_handle,
    // ];
    
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    
    let savings = counter.lock().unwrap().balance();
   
    println!("Balance: {:?}", savings);
}