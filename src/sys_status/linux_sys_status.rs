use std::fs;
use crate::sys_status::sys_status::{SysStatus,MemUsage};
pub struct LinuxStatus;

impl SysStatus for LinuxStatus {

    fn get_cpu_temperature(&self) -> Result<f32, String>{
        let temp_str = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
            .map_err(|e| e.to_string())?;
        let temp = temp_str.trim().parse::<f32>()
            .map_err(|e| e.to_string())?;
        Ok(temp / 1000.0) 
    }

    fn get_memory_usage(&self) -> Result<MemUsage, String>{
        let meminfo_string = fs::read_to_string("/proc/meminfo")
            .map_err(|e| e.to_string())?;
        
        let mut mem_usage: MemUsage = MemUsage{ mem_total:0, mem_free:0 };
        
        let total_mem_string = meminfo_string
            .lines()
            .find(|line| line.trim_start().starts_with("MemTotal:"))
            .and_then(|line| line.split_once(':').map(|(_,word)| word.trim().to_string() ) );
        
        let free_mem_string = meminfo_string
            .lines()
            .find(|line| line.trim_start().starts_with("MemFree:"))
            .and_then(|line| line.split_once(':').map(|(_,word)| word.trim().to_string() ) );

        let total_mem = total_mem_string.unwrap().parse::<u32>().map_err(|e| e.to_string())?;        
        let free_mem = free_mem_string.unwrap().parse::<u32>().map_err(|e| e.to_string())?;        

        mem_usage.mem_free = free_mem;
        mem_usage.mem_total = total_mem;

        Ok(mem_usage) // Placeholder

    }

    fn get_disk_usage(&self) -> Result<u64, String>{
        Ok(0) // Placeholder
    }

}
