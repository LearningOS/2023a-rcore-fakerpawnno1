//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,
        get_current_syscall_times, get_current_status, get_current_start_time,get_current_physical_address,mmap,unmap
    },
    timer::get_time_us,
    mm::VirtAddr
   };

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us=get_time_us();
    let sec=us/1000000;
    let usec=us%1000000;
    let start = ts as *const u8;
    let k_ts =get_current_physical_address(start) as * mut TimeVal;
    unsafe {
      *k_ts= TimeVal{
        sec:sec,
        usec:usec
      }
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let vp=ti as *const u8;
    let k_ti=get_current_physical_address(vp) as * mut TaskInfo;
    unsafe{
        *k_ti=TaskInfo{
            status: get_current_status(),
            syscall_times: get_current_syscall_times(),
            time: (get_time_us()-get_current_start_time())/1000
        }
    }
    0
}

/// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    // 页属性错误
    if port & !0x7 as usize != 0 || port & 0x7 as usize == 0 {
        return -1;
    }
    // 虚拟地址没有对齐 
    if VirtAddr::from(start).page_offset()!=0 {
        return -1;
    }
    mmap(start,len,port)
}

/// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    // 虚拟地址没有对齐 
    if VirtAddr::from(start).page_offset()!=0 {
        return -1;
    }
    unmap(start, len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
