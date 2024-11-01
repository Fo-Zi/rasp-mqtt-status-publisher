pub struct MemUsage{
    pub mem_total: u32,
    pub mem_free: u32,
}

pub trait SysStatus {
    fn get_cpu_temperature(&self) -> Result<f32, String>;
    fn get_memory_usage(&self) -> Result<MemUsage, String>;
    fn get_disk_usage(&self) -> Result<u64, String>;
}