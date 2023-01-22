use hawkeye::generated::hawkeye_service_server::HawkeyeServiceServer;
use hawkeye::generated::{hawkeye_service_client::HawkeyeServiceClient, CpuStatsRequest};
use hawkeye::HawkeyeServiceImpl;
use systemstat::{Platform, System};
use tokio::time::{sleep, Duration};
use tonic::transport::Server;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[ignore]
async fn fetch_cpu_stats() -> Result<(), Box<dyn std::error::Error>> {
    let system = System::new();
    let service = HawkeyeServiceImpl::new(system);
    let addr = "[::1]:50051".parse()?;
    tokio::spawn(async move {
        Server::builder()
            .add_service(HawkeyeServiceServer::new(service))
            .serve(addr)
            .await
            .unwrap();
    });
    sleep(Duration::from_secs(1)).await;
    let mut client = HawkeyeServiceClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(CpuStatsRequest { time: 1 });
    let response = client.get_cpu_stats(request).await?;
    assert!(response.get_ref().idle > 0.0);
    assert!(response.get_ref().idle < 1.0);
    Ok(())
}
