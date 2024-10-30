use std::{thread, cell::{Cell, RefCell}, collections::HashMap, sync::{Arc, Mutex}};

use crate::future::Future;
use crate::runtime::waker::Waker;

type Task = Box<dyn Future<Output = String>>;

// thread_local! allows us define a static variable that is uniqie to the thread it has first called from.
// This means all threads will have their own instance of such static variable.
thread_local! {
    static EXECUTOR: ExecutorInner = ExecutorInner::default();
}

#[derive(Default)]
struct ExecutorInner {
    tasks: RefCell<HashMap<usize, Task>>,
    ready_queue: Arc<Mutex<Vec<usize>>>,
    // this assigns uniqie id to each top level future
    next_id: Cell<usize>,
}

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Self{}
    }

    fn pop_ready (&self) -> Option<usize> {
        EXECUTOR.with(|e| {
            e.ready_queue.lock().map(|mut q| q.pop()).unwrap()
        })
    }

    fn remove_task(&self, id: usize) -> Option<Task> {
        EXECUTOR.with(|e| {
            e.tasks.borrow_mut().remove(&id)
        })
    }

    fn get_waker(&self, id: usize) -> Waker {
        Waker {
            id,
            thread: thread::current(),
            ready_queue: EXECUTOR.with(|e| e.ready_queue.clone())
        }   
    }

    fn add_task(&self, id: usize, task: Task) {
        EXECUTOR.with(|e| {
            e.tasks.borrow_mut().insert(id, task)
        });
    }

    fn count_tasks(&self) -> usize {
        EXECUTOR.with(|e| {
            e.tasks.borrow().len()
        })
    }

    pub fn spawn<F>(&self, fut: F)
    where
        F: Future<Output = String> + 'static
    {
        EXECUTOR.with(|e| {
            let id = e.next_id.get();
            e.tasks.borrow_mut().insert(id, Box::new(fut));
            e.ready_queue.lock().map(|mut q| q.push(id)).unwrap();
            e.next_id.set(id + 1);
        });
    }

    // It is an entry point to Executor
    pub fn block_on(&self) {
        loop {
            while let Some(id) = self.pop_ready() {
                let mut task = match self.remove_task(id) {
                    Some(task) => task,
                    // if it was false wake up - just continue
                    None => continue,
                };

                let waker = self.get_waker(id);

                match task.poll(&waker) {
                    std::task::Poll::Ready(resp) => {
                        println!("Result: {}", resp)
                    },
                    std::task::Poll::Pending => self.add_task(id, task),
                }
            }

            let pending_tasks = self.count_tasks();
            let tid = thread::current().id();

            if pending_tasks > 0 {
                println!("Thread {tid:?} has {pending_tasks} pending tasks. Sleep until new events.");
                thread::park();
            }
            else {
                println!("Thread {tid:?} has 0 pending tasks. All task ar finished.");
                break;
            }
        }
    }
}