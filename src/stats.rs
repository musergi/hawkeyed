use async_trait::async_trait;

#[async_trait]
trait StatsReader {
    async fn cpu_stats() -> CpuStats;
}

pub struct CpuStats {
    pub cpus: Vec<CpuInfo>,
}

pub struct CpuInfo {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
}
