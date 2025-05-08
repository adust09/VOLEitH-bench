use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::{fs, path::Path};
use sysinfo::{System, SystemExt};

// Constants for buffer sizes
const INITIAL_BUFFER_CAPACITY: usize = 1024 * 1024; // 1MB initial capacity for buffers

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub proof_generation_time_ms: u64,
    pub proof_verification_time_ms: u64,
    pub proof_size_bytes: usize,
    pub communication_overhead_bytes: usize,
    pub prover_cpu_usage: f32,
    pub prover_memory_usage_mb: f64,
    pub verifier_cpu_usage: f32,
    pub verifier_memory_usage_mb: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct MonitoringConfig {
    pub enabled: bool,
    pub refresh_interval_ms: u64,
    pub stabilization_delay_ms: u64,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            refresh_interval_ms: 50,     // Reduced from 100ms
            stabilization_delay_ms: 100, // Reduced from 200ms
        }
    }
}

// Pre-allocate a buffer with capacity for reuse
pub fn create_buffer_with_capacity(data: &[u8]) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(INITIAL_BUFFER_CAPACITY.max(data.len()));
    buffer.extend_from_slice(data);
    buffer
}

// Global cache for file contents to avoid repeated file I/O
static FILE_CACHE: Lazy<Mutex<HashMap<String, Vec<u8>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// Read file contents efficiently, using cache when possible
/// Returns file content as bytes
pub fn read_file_cached(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    let path_str = path.to_string_lossy().to_string();

    // Try to get from cache first
    {
        let cache = FILE_CACHE.lock().unwrap();
        if let Some(content) = cache.get(&path_str) {
            return Ok(content.clone());
        }
    }

    // Not in cache, read from disk
    let content = fs::read(path)?;

    // Store in cache for future use
    {
        let mut cache = FILE_CACHE.lock().unwrap();
        cache.insert(path_str, content.clone());
    }

    Ok(content)
}

/// Initialize system monitoring and get the initial CPU/memory values
pub fn init_system_monitoring(config: &MonitoringConfig) -> Option<System> {
    if !config.enabled {
        return None;
    }

    let mut system = System::new_all();

    // Only refresh specific components we need instead of all
    system.refresh_processes();
    system.refresh_memory();
    system.refresh_cpu();

    // Sleep to allow initial measurements to settle (configurable)
    if config.stabilization_delay_ms > 0 {
        std::thread::sleep(std::time::Duration::from_millis(config.stabilization_delay_ms));
    }

    // Must refresh again to establish the baseline for CPU measurements
    system.refresh_processes();
    system.refresh_cpu();

    Some(system)
}
