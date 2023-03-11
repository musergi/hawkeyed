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
                StatLine::CpuAggregate(metrics) => report_metrics("agg", metrics),
                StatLine::Cpu(n, metrics) => report_metrics(&n.to_string(), metrics),
            }
        }
        println!("Writen metric");
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

fn report_metrics(prefix: &str, metrics: CpuMetric) {
    let label = [("cpu_id", format!("{}", prefix))];
    absolute_counter!("cpu_user", metrics.user, &label);
    absolute_counter!("cpu_nice", metrics.nice, &label);
    absolute_counter!("cpu_system", metrics.system, &label);
    absolute_counter!("cpu_idle", metrics.idle, &label);
    absolute_counter!("cpu_iowait", metrics.iowait, &label);
    absolute_counter!("cpu_irq", metrics.irq, &label);
    absolute_counter!("cpu_softirq", metrics.softirq, &label);
}
