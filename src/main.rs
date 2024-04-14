use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Mutex;
use std::rc::Rc;

#[allow(unused)]
fn two_task_at_once(){
    let manipulator = thread::spawn(|| {
        for i in 1..10{
            println!("hello n°{} from new task",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    manipulator.join().unwrap();
    for i in 1..5 {
        println!("Hello n°{} from current task",i);
        thread::sleep(Duration::from_millis(1));
    }

}

#[allow(unused)]
fn using_move_on_new_task(){
    let v = vec![13,14,182];

    let manipulator = thread::spawn(move|| {
       println!("that's the vector {:?}",v)
    });

    manipulator.join().unwrap()
}

#[allow(unused)]
fn channel_creation(){
    let (tx,rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
       let values = vec![
           String::from("It is"),
           String::from("messages from"),
           String::from("a new task"),
           String::from("with same"),
           String::from("receiver")
       ];

        for value in values {
            tx1.send(value).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    thread::spawn(move || {
        let values = vec![
            String::from("Hey"),
            String::from("I am"),
            String::from("a message"),
            String::from("to"),
            String::from("you"),
        ];
        for value in values {
            tx.send(value).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    for received in rx {
        println!("We got : {}",received)
    }
}

#[allow(unused)]
fn mutex_introduction(){
    let m = Mutex::new(5i64);
    {
        let mut number = m.lock().unwrap();
        *number = 6;
    }

    println!("Number equal {:?}",m)
}

fn counter_mutex(){
    let counter = Rc::new(Mutex::new(0));
    let mut manipulators = vec![];

    for _ in 1..10 {
        let counter = Rc::clone(&counter);
        let manipulator = thread::spawn(move || {
           let mut number = counter.lock().unwrap();
            *number += 1;
        });
        manipulators.push(manipulator)
    }

    for manipulator in manipulators {
        manipulator.join().unwrap();
    }

    println!("Result : {}", *counter.lock().unwrap());
}


fn main() {
    counter_mutex();
}