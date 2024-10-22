use std::{io::{self, Result}, net::TcpStream, os::fd::AsRawFd};
use crate::ffi;

type Events = Vec<ffi::Event>;

/// `Registry` is a handle that allows us to register interest in new events.
pub struct Registry {
    raw_fd: i32,
}

impl Registry {
    pub fn register(&self, source: &TcpStream, token: usize, interest: i32) -> Result<()> {
        todo!()
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        todo!()
    }
}

/// `Poll` is a struct that represents the event queue itself.<br>
/// 
/// It has a few methods:
/// - `new`: creates new event queue;
/// - `registry`: returns a referenece to the registry that we can use to register interest to be notified about new events;
/// - `poll`: blocks the thread it's called on until an event is ready or it timesout, whichever occurs first;
pub struct Poll {
    registry: Registry,
}

impl Poll {
    pub fn new() -> Result<Self> {
        todo!()
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn poll(&mut self, events: &mut Events, timeout: Option<i32>) -> Result<()> {
        todo!()
    }
}