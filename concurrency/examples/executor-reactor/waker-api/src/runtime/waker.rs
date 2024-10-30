use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::{self, Thread}
};


#[derive(Clone)]
pub struct Waker {
    pub thread: Thread,
    pub id: usize, // task id this waker is associated with
    pub ready_queue: Arc<Mutex<Vec<usize>>>, // reference that can be shared between threads
}

impl Waker {
    pub fn wake(&self) {
        // self.id is an id of task
        // the code below puts task id to ready queue
        self.ready_queue.lock().map(|mut q| q.push(self.id)).unwrap();
        // then wake up
        self.thread.unpark();
    }
}

// 
