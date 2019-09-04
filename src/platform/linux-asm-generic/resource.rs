use super::time::*;
use super::types::*;

/// Resource control/accounting header file for linux

/// Definition of struct rusage taken from BSD 4.3 Reno
///
/// We don't support all of these yet, but we might as well have them....
/// Otherwise, each time we add new items, programs which depend on this
/// structure will lose.  This reduces the chances of that happening.
pub const RUSAGE_SELF: i32 = 0;
pub const RUSAGE_CHILDREN: i32 = -1;
/// sys_wait4() uses this
pub const RUSAGE_BOTH: i32 = -2;
/// only the calling thread
pub const RUSAGE_THREAD: i32 = 1;

#[repr(C)]
pub struct rusage_t {
    /// user time used
    pub ru_utime: timeval_t,
    /// system time used
    ru_stime: timeval_t,
    /// maximum resident set size
    pub ru_maxrss: isize,
    /// integral shared memory size
    pub ru_ixrss: isize,
    /// integral unshared data size
    pub ru_idrss: isize,
    /// integral unshared stack size
    pub ru_isrss: isize,
    /// page reclaims
    pub ru_minflt: isize,
    /// page faults
    pub ru_majflt: isize,
    /// swaps
    pub ru_nswap: isize,
    /// block input operations
    pub ru_inblock: isize,
    /// block output operations
    pub ru_oublock: isize,
    /// messages sent
    pub ru_msgsnd: isize,
    /// messages received
    pub ru_msgrcv: isize,
    /// signals received
    pub ru_nsignals: isize,
    /// voluntary context switches
    pub ru_nvcsw: isize,
    /// involuntary
    pub ru_nivcsw: isize,
}

#[repr(C)]
pub struct rlimit_t {
    pub rlim_cur: usize,
    pub rlim_max: usize,
}

// TODO(Shaohua): Fix type error
//pub const RLIM64_INFINITY: u64 =	~0;

#[repr(C)]
pub struct rlimit64_t {
    pub rlim_cur: u64,
    pub rlim_max: u64,
}

pub const PRIO_MIN: i32 = -20;
pub const PRIO_MAX: i32 = 20;

pub const PRIO_PROCESS: i32 = 0;
pub const PRIO_PGRP: i32 = 1;
pub const PRIO_USER: i32 = 2;

/// Limit the stack by to some sane default: root can always
/// increase this limit if needed..  8MB seems reasonable.
pub const _STK_LIM: i32 = (8 * 1024 * 1024);

// GPG2 wants 64kB of mlocked memory, to make sure pass phrases
// and other sensitive information are never written to disk.
//TODO(Shaohua): PAGE_SIZE undefined
//pub const MLOCK_LIMIT: i32 = 	((PAGE_SIZE > 64*1024) ? PAGE_SIZE : 64*1024);

// Due to binary compatibility, the actual resource numbers
// may be different for different linux versions..
