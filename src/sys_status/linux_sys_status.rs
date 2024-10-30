mod sys_status;
use std::fs;

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
        Ok(0) // Placeholder
    }

    fn get_disk_usage(&self) -> Result<u64, String>{
        Ok(0) // Placeholder
    }

    fn network_status(&self) -> Result<String, String> {
        Ok("Connected".to_string()) // Placeholder
    }

}
