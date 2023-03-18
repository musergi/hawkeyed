use hawkeye::{CpuMetric, MemoryLine, StatLine};
use metrics::{absolute_counter, gauge};
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;
use std::time::Duration;

#[derive(Debug, Default)]
struct MemMetrics {
    free: Option<u64>,
    total: Option<u64>,
}

#[tokio::main]
async fn main() {
    PrometheusBuilder::new()
        .with_http_listener("0.0.0.0:50500".parse::<SocketAddr>().unwrap())
        .install()
        .expect("failed to install exporter");
    loop {
        let file_content = tokio::fs::read_to_string("/proc/stat").await.unwrap();
        let stats = file_content
            .lines()
            .filter_map(|line| line.parse::<StatLine>().ok());
        for stat in stats {
            match stat {
                StatLine::CpuAggregate(metrics) => report_cpu_metrics("agg", metrics),
                StatLine::Cpu(n, metrics) => report_cpu_metrics(&format!("cpu_{}", n), metrics),
            }
        }
        let file_content = tokio::fs::read_to_string("/proc/meminfo").await.unwrap();
        let stats = file_content
            .lines()
            .filter_map(|line| line.parse::<MemoryLine>().ok());
        let mut metrics = MemMetrics::default();
        for stat in stats {
            match stat {
                MemoryLine::Free(v) => metrics.free = Some(v),
                MemoryLine::Total(v) => metrics.total = Some(v),
            }
        }
        report_mem_metrics(metrics);
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

fn report_cpu_metrics(cpu_id: &str, metrics: CpuMetric) {
    let label = [("cpu_id", format!("{}", cpu_id))];
    absolute_counter!("cpu_user", metrics.user, &label);
    absolute_counter!("cpu_nice", metrics.nice, &label);
    absolute_counter!("cpu_system", metrics.system, &label);
    absolute_counter!("cpu_idle", metrics.idle, &label);
    absolute_counter!("cpu_iowait", metrics.iowait, &label);
    absolute_counter!("cpu_irq", metrics.irq, &label);
    absolute_counter!("cpu_softirq", metrics.softirq, &label);
}

fn report_mem_metrics(metrics: MemMetrics) {
    let MemMetrics { free, total } = metrics;
    match (free, total) {
        (Some(free), Some(total)) => {
            gauge!("mem_free", free as f64);
            gauge!("mem_total", total as f64);
        }
        _ => println!("Could not write mem metrics"),
    }
}
