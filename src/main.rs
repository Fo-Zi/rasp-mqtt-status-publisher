use std::fs;
use std::thread;
use std::time::Duration;

fn get_cpu_temperature() -> Result<f32, Box<dyn std::error::Error>> {
    // Read the temperature from the system file
    let temp_str = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")?;
    
    // Parse the temperature as an integer and divide by 1000 to get Celsius
    let temp_millidegrees: i32 = temp_str.trim().parse()?;
    let temp_celsius = temp_millidegrees as f32 / 1000.0;

    Ok(temp_celsius)
}

fn main() {
    loop {
        match get_cpu_temperature() {
            Ok(temp) => println!("CPU Temperature: {:.1}°C", temp),
            Err(e) => eprintln!("Failed to read CPU temperature: {}", e),
        }
        thread::sleep(Duration::from_secs(2)); 
    }
}