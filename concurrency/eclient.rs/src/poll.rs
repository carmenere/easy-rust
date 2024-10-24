use std::{io::{self, Result}, net::TcpStream, os::fd::AsRawFd, sync::mpsc::RecvTimeoutError};
use crate::ffi;

type Events = Vec<ffi::Event>;

/// `Registry` is a handle that allows us to register interest in new events.
pub struct Registry {
    epfd: i32,
}

impl Registry {
    pub fn register(&self, source: &TcpStream, token: usize, interest: i32) -> Result<()> {
        let mut event = ffi::Event {
            events: interest as u32,
            data: token,
        };

        let op = ffi::EPOLL_CTL_ADD;
        let res = unsafe {
            ffi::epoll_ctl(self.epfd, op, source.as_raw_fd(), &mut event)
        };

        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        let res = unsafe {
            ffi::close(self.epfd)
        };
        // Adding panic to drop is a bad idea, because drop can be called within a panic already, which cause the process to abort.
        // So, we just get and print to stderr the `errno` expplicitly.
        if res < 0 {
            let err = io::Error::last_os_error();
            eprint!("ERROR: {:?}", err); 
        }
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
        
        let res = unsafe {
            ffi::epoll_create(1)
        };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(Self {registry: Registry{epfd: res}})
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn poll(&mut self, events: &mut Events, timeout: Option<i32>) -> Result<()> {
        let fd = self.registry.epfd;
        let timeout = timeout.unwrap_or(-1);
        let max_events = events.capacity() as i32;
        let res = unsafe {
            ffi::epoll_wait(fd, events.as_mut_ptr(), max_events, timeout)
        };

        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        // This is unsafe since we could set the length so that we could access memory that is not been initialized yet in safe Rust.
        // The OS guarantees that number of events it returns is pointing to valid data in our Vec.
        unsafe { events.set_len(res as usize)};

        Ok(())
    }
}