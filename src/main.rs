#![no_std]
#![no_main]
#![doc = include_str!("../README.md")]

extern crate alloc;
#[macro_use]
extern crate axlog;

mod syscall;

use alloc::vec::Vec;
use starry_core::entry::run_user_app;

#[unsafe(no_mangle)]
fn main() {
    let testcases = option_env!("AX_TESTCASES_LIST")
        .unwrap_or_else(|| "Please specify the testcases list by making user_apps")
        .split(',')
        .filter(|&x| !x.is_empty());

    for testcase in testcases {
        let args = testcase
            .split_ascii_whitespace()
            .map(Into::into)
            .collect::<Vec<_>>();

        info!(
            "[task manager] Running user task: {} with args: {:?}",
            testcase, args
        );

        let exit_code = run_user_app(&args, &[]);
        info!(
            "[task manager] User task {} exited with code: {:?}",
            testcase, exit_code
        );
    }
}
