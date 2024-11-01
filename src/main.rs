use std::thread;
use std::time::Duration;

mod sys_status;
use sys_status::sys_status::SysStatus;

use crate::sys_status::linux_sys_status::LinuxStatus;

fn main() {
    let linux_status = LinuxStatus {};
    loop {
        match linux_status.get_cpu_temperature(){
            Ok(temp) => println!("CPU Temp: {}",temp),
            Err(e) => println!("{e}"),
        };
        match linux_status.get_memory_usage(){
            Ok(mem_usage) => println!("Mem usage - Total:{} , Free: {}",mem_usage.mem_total,mem_usage.mem_free),
            Err(e) => println!("{e}"),
        };
        thread::sleep(Duration::from_secs(2)); 
    }
}