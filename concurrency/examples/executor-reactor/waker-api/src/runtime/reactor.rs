use std::{collections::HashMap, sync::{atomic::{AtomicUsize, Ordering}, Arc, Mutex, OnceLock}, time::Duration, thread};
use mio::{Poll, net::TcpStream, Events, Interest, Registry, Token};
use super::waker::Waker;

type Wakers = Arc<Mutex<HashMap<usize, Waker>>>;

// This static variable is accessible from different threads
static REACTOR: OnceLock<Reactor> = OnceLock::new();

pub fn reactor() -> &'static Reactor {
    REACTOR.get().expect("Reactor hasn't initialized yet.")
}

pub struct Reactor {
    wakers: Wakers,
    registry: Registry,
    next_id: AtomicUsize,
}

impl Reactor {
    pub fn register(&self, stream: &mut TcpStream, interest: Interest, id: usize) {
        self.registry.register(stream, Token(id), interest).unwrap();
    }

    pub fn deregister(&self, stream: &mut TcpStream, id: usize) {
        self.wakers.lock().map(|mut hm| {hm.remove(&id)}).unwrap();
        self.registry.deregister(stream).unwrap();
    }

    pub fn add_waker(&self, waker: &Waker, id: usize) {
        self.wakers.lock().map(|mut hm| {hm.insert(id, waker.clone())}).unwrap();
    }

    pub fn next_id(&self) -> usize {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }
}

pub fn event_loop(mut poll: Poll, wakers: Wakers) {
    let mut events = Events::with_capacity(512);

    loop {
        poll.poll(&mut events, None).unwrap();
        for e in events.iter() {
            let Token(id) = e.token();
            let wakers = wakers.lock().unwrap();
            wakers.get(&id).map(|w| {w.wake()});
        }
    }
}

pub fn start() {
    let wakers = Arc::new(Mutex::new(HashMap::new()));

    let poll = Poll::new().unwrap();
    let registry = poll.registry().try_clone().unwrap();
    let next_id = AtomicUsize::new(1);

    let reactor = Reactor {
        wakers: wakers.clone(),
        registry,
        next_id
    };

    REACTOR.set(reactor).ok().expect("Reactor has already run.");

    // new separate OS thread for event_loop
    thread::spawn(|| {
        event_loop(poll, wakers);
    });
}
