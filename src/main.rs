use hawkeye::{CpuMetric, StatLine};
use metrics::absolute_counter;
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;
use std::time::Duration;

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
                StatLine::CpuAggregate(metrics) => report_metrics("cpu", metrics),
                _ => (),
            }
        }
        println!("Writen metric");
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

fn report_metrics(prefix: &str, metrics: CpuMetric) {
    absolute_counter!(format!("{}_{}", prefix, "user"), metrics.user);
    absolute_counter!(format!("{}_{}", prefix, "nice"), metrics.nice);
    absolute_counter!(format!("{}_{}", prefix, "system"), metrics.system);
    absolute_counter!(format!("{}_{}", prefix, "idle"), metrics.idle);
    absolute_counter!(format!("{}_{}", prefix, "iowait"), metrics.iowait);
    absolute_counter!(format!("{}_{}", prefix, "irq"), metrics.irq);
    absolute_counter!(format!("{}_{}", prefix, "softirq"), metrics.softirq);
}
