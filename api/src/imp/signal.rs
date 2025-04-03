use core::ffi::c_void;

use axerrno::LinuxResult;

use crate::ptr::{UserConstPtr, UserPtr};

pub fn sys_rt_sigprocmask(
    _how: i32,
    _set: UserConstPtr<c_void>,
    _oldset: UserPtr<c_void>,
    _sigsetsize: usize,
) -> LinuxResult<isize> {
    warn!("sys_rt_sigprocmask: not implemented");
    Ok(0)
}

pub fn sys_rt_sigaction(
    _signum: i32,
    _act: UserConstPtr<c_void>,
    _oldact: UserPtr<c_void>,
    _sigsetsize: usize,
) -> LinuxResult<isize> {
    warn!("sys_rt_sigaction: not implemented");
    Ok(0)
}

// TODO: [stub] The method signature is not correct yet
pub fn sys_rt_sigtimedwait(
    _signum: i32,
    _act: UserConstPtr<c_void>,
    _old_act: UserPtr<c_void>,
    _sig_set_size: usize,
) -> LinuxResult<isize> {
    warn!("[sys_rt_sigaction] not implemented yet");
    Ok(0)
}
