use std::thread;
use std::time::Duration;
use std::sync::mpsc;

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

    thread::spawn(move || {
        let value = String::from("Hey");
        tx.send(value).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("We received {}",received)
}



fn main() {
    channel_creation();
}