use std::thread;
use std::time::Duration;

fn two_task_at_once(){
    thread::spawn(|| {
        for i in 1..10{
            println!("hello n°{} from new task",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("Hello n°{} from current task",i);
        thread::sleep(Duration::from_millis(1));
    }

}


fn main() {
    two_task_at_once()
}