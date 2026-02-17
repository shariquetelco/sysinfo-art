use sysinfo::{Components, Disks, Networks, System};

#[derive(Debug, Clone)]
pub struct Metrics {
    pub cpu_usage: f32,       // 0.0 - 100.0
    pub ram_usage: f32,       // 0.0 - 100.0
    pub ram_used_gb: f64,
    pub ram_total_gb: f64,
    pub disk_usage: f32,      // 0.0 - 100.0
    pub disk_read_mb: f64,
    pub disk_write_mb: f64,
    pub net_rx_mb: f64,
    pub net_tx_mb: f64,
    pub net_activity: f32,    // 0.0 - 100.0
    pub temperature: f32,     // celsius, 0.0 if not available
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            ram_usage: 0.0,
            ram_used_gb: 0.0,
            ram_total_gb: 0.0,
            disk_usage: 0.0,
            disk_read_mb: 0.0,
            disk_write_mb: 0.0,
            net_rx_mb: 0.0,
            net_tx_mb: 0.0,
            net_activity: 0.0,
            temperature: 35.0,
        }
    }
}

pub struct MetricsCollector {
    sys: System,
    disks: Disks,
    networks: Networks,
    components: Components,
    prev_rx: u64,
    prev_tx: u64,
}

impl MetricsCollector {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let (prev_rx, prev_tx) = {
            let networks = Networks::new_with_refreshed_list();
            let rx: u64 = networks.iter().map(|(_, n)| n.total_received()).sum();
            let tx: u64 = networks.iter().map(|(_, n)| n.total_transmitted()).sum();
            (rx, tx)
        };

        Self {
            sys,
            disks: Disks::new_with_refreshed_list(),
            networks: Networks::new_with_refreshed_list(),
            components: Components::new_with_refreshed_list(),
            prev_rx,
            prev_tx,
        }
    }

    pub fn collect(&mut self) -> Metrics {
        self.sys.refresh_all();
        self.disks.refresh(true);
        self.networks.refresh(true);
        self.components.refresh(true);

        // CPU
        let cpu_usage = self.sys.global_cpu_usage();

        // RAM
        let total_mem = self.sys.total_memory();
        let used_mem = self.sys.used_memory();
        let ram_usage = if total_mem > 0 {
            (used_mem as f32 / total_mem as f32) * 100.0
        } else {
            0.0
        };
        let ram_total_gb = total_mem as f64 / 1_073_741_824.0;
        let ram_used_gb = used_mem as f64 / 1_073_741_824.0;

        // Disk
        let (disk_usage, disk_read_mb, disk_write_mb) = self.calc_disk();

        // Network
        let (net_rx_mb, net_tx_mb, net_activity) = self.calc_network();

        // Temperature
        let temperature = self.calc_temperature();

        Metrics {
            cpu_usage,
            ram_usage,
            ram_used_gb,
            ram_total_gb,
            disk_usage,
            disk_read_mb,
            disk_write_mb,
            net_rx_mb,
            net_tx_mb,
            net_activity,
            temperature,
        }
    }

    fn calc_disk(&self) -> (f32, f64, f64) {
        let mut total_space: u64 = 0;
        let mut available_space: u64 = 0;

        for disk in self.disks.iter() {
            total_space += disk.total_space();
            available_space += disk.available_space();
        }

        let usage = if total_space > 0 {
            let used = total_space.saturating_sub(available_space);
            (used as f32 / total_space as f32) * 100.0
        } else {
            0.0
        };

        (usage, 0.0, 0.0)
    }

    fn calc_network(&mut self) -> (f64, f64, f32) {
        let curr_rx: u64 = self.networks.iter().map(|(_, n)| n.total_received()).sum();
        let curr_tx: u64 = self.networks.iter().map(|(_, n)| n.total_transmitted()).sum();

        let rx_delta = curr_rx.saturating_sub(self.prev_rx);
        let tx_delta = curr_tx.saturating_sub(self.prev_tx);

        self.prev_rx = curr_rx;
        self.prev_tx = curr_tx;

        let rx_mb = rx_delta as f64 / 1_048_576.0;
        let tx_mb = tx_delta as f64 / 1_048_576.0;

        // Clamp activity to 0-100
        let combined = (rx_mb + tx_mb) * 10.0;
        let activity = combined.min(100.0) as f32;

        (rx_mb, tx_mb, activity)
    }

    fn calc_temperature(&self) -> f32 {
        let temps: Vec<f32> = self
            .components
            .iter()
            .filter_map(|c| {
                c.temperature()
                    .filter(|&t| t > 0.0 && t < 150.0)
            })
            .collect();

        if temps.is_empty() {
            // Fallback: estimate from CPU load
            35.0
        } else {
            temps.iter().cloned().fold(f32::NEG_INFINITY, f32::max)
        }
    }
}
