// use std::future::Future;
// use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

// use once_cell::sync::Lazy;

// use tokio::runtime::{Builder, Runtime};
// use tokio::task::JoinHandle;
// use tokio::sync::mpsc::Sender;



// static SCHEDULER: Lazy<RwLock<Runtime>> = Lazy::new(|| {
//     let built = Builder::new()
//         .threaded_scheduler()
//         .build()
//         .unwrap_or_else(|e| {
//             panic!("opencl-runtime: Failed to start tokio runtime due to {:?}", e)
//         });
//     RwLock::new(built)
// });

// fn read_lock<'a>() -> RwLockReadGuard<'a, Runtime> {
//     SCHEDULER.read().unwrap()
// }

// fn write_lock<'a>() -> RwLockWriteGuard<'a, Runtime> {
//     SCHEDULER.write().unwrap()
// }

// pub fn spawn<T>(task: T) -> JoinHandle<T::Output>
// where
//     T: Future + Send + 'static,
//     T::Output: Send + 'static,
// {
//     read_lock().spawn(task)
// }

// pub struct Tx<M: Send> {
//     chan: Arc<Mutex<Sender<M>>>
// }

// impl<M: Send + 'static> Tx<M> {
//     pub fn new(sender: Sender<M>) -> Tx<M> {
//         Tx{ chan: Arc::new(Mutex::new(sender)) }
//     }

//     pub fn send(&self, message: M) {
//         let lock: MutexGuard<Sender<M>> = self.chan.lock().expect("Tx failed to obtain a lock");
//         let mut sender = lock.clone();
//         spawn(async move {
//             match sender.send(message).await {
//                 Ok(_) => (),
//                 Err(_err) => trace!("Tx send error"),
//             }
//         });
//     }
// }



// #[cfg(test)]
// mod tests {
//     use crate::scheduler;
//     use scheduler::{Tx};
//     use tokio::sync::mpsc::{channel, Receiver};
//     use tokio::task::JoinHandle;
   
//     enum Msg {
//         Add(u32),
//         Stop,
//     }
    
//     fn spawn_some_task(mut rx: Receiver<Msg>) -> JoinHandle<u32> {
//         scheduler::spawn(async move {
//             let mut total: u32 = 0;
//             loop {
//                 match rx.recv().await {
//                     Some(Msg::Add(num)) => total += num,
//                     Some(Msg::Stop) => break,
//                     None => continue,
//                 }
//             }
//             total
//         })
//     }


//     fn start_a_task() -> (JoinHandle<u32>, Tx<Msg>) {
//         let (tx, rx) = channel::<Msg>(1000);
//         let handle = spawn_some_task(rx);
//         (handle, Tx::new(tx))
//     }

//     #[tokio::test]
//     async fn task_can_be_spawned_messaged_and_awaited() {
//         let (handle, tx) = start_a_task();
        
//         tx.send(Msg::Add(20));
//         tx.send(Msg::Add(20));
//         tx.send(Msg::Stop);

//         // std::thread::sleep(std::time::Duration::from_millis(1));
//         let total = handle.await.expect("Failed to await");
//         assert_eq!(total, 40);
//     }
// }
