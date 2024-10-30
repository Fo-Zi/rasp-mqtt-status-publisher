pub struct MemUsage{
    mem_total: u32,
    mem_free: u32,
}

pub trait SysStatus {
    fn get_cpu_temperature(&self) -> Result<f32, String>;
    fn get_memory_usage(&self) -> Result<MemUsage, String>;
    fn get_disk_usage(&self) -> Result<u64, String>;
}