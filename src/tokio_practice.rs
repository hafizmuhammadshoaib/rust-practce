// use futures::future::join_all;
use std::time::Duration;

use futures::future::try_join_all;
use thiserror::Error;
use tokio::{task::JoinHandle, time::sleep};

#[derive(Error, Debug)]
enum MyError {
    #[error("Network Error")]
    MyNetworkError,
}

struct Task {
    task: Vec<i32>,
}

#[tokio::main]
pub async fn main() {
    let mut handles = Vec::new();
    for i in 1..5 {
        handles.push(scheduler(i, 1).await);
    }
    match try_join_all(handles).await {
        Ok(_) => println!("All tasks finished successfully"),
        Err(e) => eprintln!("Error in tasks: {}", e),
    }
}

// async fn performing_sleep_in_async_task() {
//     println!("Hello");
//     tokio::time::sleep(Duration::from_secs(2)).await;
//     println!("after 2 secs");
// }

// async fn spawning_multiple_async_tasks() {
//     let handles = [
//         tokio::spawn(some_async_task(3, 1)),
//         tokio::spawn(some_async_task(2, 2)),
//     ];

//     match try_join_all(handles).await {
//         Ok(_) => println!("task completed successfully"),
//         Err(e) => eprintln!("task failed {:?}", e),
//     }
// }

async fn some_async_task(time: u64, task_no: i32) -> () {
    tokio::time::sleep(Duration::from_secs(time)).await;
    println!("completed {}", task_no);
}

// async fn message_passing_mpsc_example() {
//     let (tx, mut rx) = mpsc::channel(10);

//     let producer = tokio::spawn(async move {
//         for i in 0..5 {
//             let msg = format!("Message {}", i + 1);
//             tx.send(msg).await.expect("failed to send message");
//             println!("Sent: {}", i + 1);
//             tokio::time::sleep(Duration::from_secs(1)).await;
//         }
//     });

//     let consumer = tokio::spawn(async move {
//         while let Some(msg) = rx.recv().await {
//             println!("Received: {}", msg);
//         }
//     });

//     let (producer_result, consumer_result) = tokio::join!(producer, consumer);
//     if let Err(e) = producer_result {
//         eprint!("producer error {}", e);
//     }

//     if let Err(e) = consumer_result {
//         eprint!("consumer error {}", e);
//     }
// }

// async fn shared_counter() {
//     let shared_counter = Arc::new(Mutex::new(0u32));

//     let mut handles = vec![];

//     for i in 0..5 {
//         let counter = Arc::clone(&shared_counter);
//         handles.push(tokio::spawn(increment_counter_task(counter)));
//     }

//     match try_join_all(handles).await {
//         Ok(_) => println!("{}", *shared_counter.lock().await),
//         Err(e) => eprint!("{}", e),
//     };
// }

// async fn increment_counter_task(shared_counter: Arc<Mutex<u32>>) {
//     let mut num = shared_counter.lock().await;
//     *num += 1;
// }

// async fn retry_with_timeout() {
//     for i in 0..3 {
//         println!("doing network call {} time", i + 1);
//         let result = simulate_network_call().await;
//         if let Ok(1) = result {
//             println!("call successful");
//             return;
//         } else if let Err(e) = result {
//             println!("call failed with error: {:?}! Retrying...", e);
//         } else {
//             println!("got unexpected success value, retrying...");
//         }
//         tokio::time::sleep(Duration::from_secs(1)).await;
//     }
// }

// async fn simulate_network_call() -> Result<i32, MyError> {
//     tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
//     let mut rng = rand::thread_rng();
//     let random_bit = rng.gen_range(0..=1);
//     println!(" random_bit {}", random_bit);
//     if random_bit == 1 {
//         return Ok(random_bit);
//     }
//     return Err(MyError::MyNetworkError);
// }

// Async Task Scheduler
// ✅ Task:

// Build a scheduler that accepts tasks with a delay

// Tasks are stored and executed after delay

// Use tokio::spawn and tokio::time::sleep

async fn scheduler(new_task: i32, delay_s: u64) -> JoinHandle<()> {
    return tokio::spawn(async move {
        sleep(Duration::from_secs(delay_s)).await;
        some_async_task(delay_s, new_task).await;
    });
}
