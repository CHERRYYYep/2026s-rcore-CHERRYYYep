//! Process management syscalls
use crate::{
    task::{
        current_task_syscall_count, exit_current_and_run_next, suspend_current_and_run_next,
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

// TODO: implement the syscall
pub fn sys_trace(_trace_request: usize, id: usize, data: usize) -> isize {
    match _trace_request {
        0 => {
            trace!("kernel: sys_trace with request 0, id {}", id);
            let value = unsafe { (id as *const usize).read_volatile() };
            return value as isize;
        },
        1 => {
            trace!("kernel: sys_trace with request 1, id {}, data {}", id, data);
            unsafe {(id as *mut usize).write_volatile(data);
            };
            return 0;   
        },
        2 => {
            trace!("kernel: sys_trace with request 2, id {}", id);
            return current_task_syscall_count(id) as isize;
        },
        _ => {
            trace!("kernel: sys_trace with unknown request {}, id {}, data {}", _trace_request, id, data);
            return -1;
        } 
    }
}
