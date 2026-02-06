use sysinfo::System;
use std::sync::{Arc, Mutex};
use std::time::Instant;


#[derive(Clone, Debug, serde::Serialize)]
pub struct Metrics {
    pub cpu_usage: f32,
    pub memory_usage_kb: u64,
    pub total_memory_kb: u64,
    pub tps: f64, // Ticks Per Second
        pub concepts_in_memory: usize,
        pub power_draw_w: f32,
        // pub gpus: Vec<GpuMetrics>,
}

pub struct PerformanceMonitor {
    
    system: Arc<Mutex<System>>,
    last_tick_time: Instant,
    tick_count: u64,
}

impl PerformanceMonitor {
    pub fn new() -> Self {


        let mut sys = System::new_all();
        sys.refresh_all();
        
        PerformanceMonitor {
                        
            system: Arc::new(Mutex::new(sys)),
            last_tick_time: Instant::now(),
            tick_count: 0,
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
    }

        pub fn get_metrics(&mut self, concepts_in_memory: usize, power_draw_w: f32) -> Metrics {
                

        let mut sys = self.system.lock().unwrap();
                sys.refresh_cpu();
        sys.refresh_memory();
        

        let cpu_usage = sys.global_cpu_info().cpu_usage();
        let memory_usage_kb = sys.used_memory() / 1024;
        let total_memory_kb = sys.total_memory() / 1024;

        let elapsed = self.last_tick_time.elapsed();
        let tps = if elapsed.as_secs_f64() > 0.5 { // Calculate every half second
            let tps_value = self.tick_count as f64 / elapsed.as_secs_f64();
            self.tick_count = 0;
            self.last_tick_time = Instant::now();
            tps_value
        } else {
            0.0 // Or carry over the old value, for now 0 is fine
        };

        Metrics {
            cpu_usage,
            memory_usage_kb,
            total_memory_kb,
                        tps,
                        concepts_in_memory,
            power_draw_w,
                                                            // gpus: Vec::new(),
        }
    }
}
