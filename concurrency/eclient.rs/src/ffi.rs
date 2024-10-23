// Valid opcodes
pub const EPOLL_CTL_ADD: i32 = 1;
pub const EPOLL_CTL_DEL: i32 = 2;
pub const EPOLL_CTL_MOD: i32 = 3;

// EPOLL_EVENTS
pub const EPOLLIN: i32 = 0x1;
pub const EPOLLPRI: i32 = 0x2;
pub const EPOLLOUT: i32 = 0x4;
pub const EPOLLRDNORM: i32 = 0x40;
pub const EPOLLRDBAND: i32 = 0x80;
pub const EPOLLWRNORM: i32 = 0x100;
pub const EPOLLWRBAND: i32 = 0x200;
pub const EPOLLMSG: i32 = 0x400;
pub const EPOLLERR: i32 = 0x8;
pub const EPOLLHUP: i32 = 0x10;
pub const EPOLLRDHUP: i32 = 0x2000;
pub const EPOLLEXCLUSIVE: i32 = 1 << 28;
pub const EPOLLWAKEUP: i32 = 1 << 29;
pub const EPOLLONESHOT: i32 = 1 << 30;
pub const EPOLLET: i32 = 1 << 31;

#[link(name = "c")]
extern "C" {
    pub fn epoll_create(size: i32) -> i32;
    pub fn close(fd: i32) -> i32;
    pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut Event) -> i32;
    pub fn epoll_wait(epfd: i32, events: *mut Event, maxevents: i32, timeout: i32) -> i32;
}

/// The `Event` struct is used to communicate to the OS.
/// 
/// Fields:
/// - `events`: bitmask we pass to OS in `epoll_ctl` and receive from OS in `epoll_wait`;
/// - `data`: some data we pass to OS in `epoll_ctl` and receive from OS in `epoll_wait`;
#[derive(Debug)]
#[repr(C, packed)]
pub struct Event {
    pub(crate) events: u32,
    pub(crate) data: usize,
}

impl Event {
    pub fn token(&self) -> usize {
        self.data
    }
}
