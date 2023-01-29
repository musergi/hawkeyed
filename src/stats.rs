trait StatsReader {
	async fn cpu_stats() -> CpuStats;
}

struct CpuStats {
	cpus: Vec<CpuInfo>,
}

struct CpuInfo {
	user: u64,
	nice: u64,
	system: u64,
	idle: u64,
	iowait: u64,
	irq: u64,
	softirq: u64,
}
