use std::thread;

use generated::{hawkeye_service_server::HawkeyeService, CpuStatsRequest, CpuStatsResponse};
use systemstat::{Duration, Platform, System};
use tonic::{Request, Response, Status};

pub mod generated {
    tonic::include_proto!("hawkeye");
}

pub struct HawkeyeServiceImpl {
    system: System,
}

impl HawkeyeServiceImpl {
    pub fn new(system: System) -> HawkeyeServiceImpl {
        HawkeyeServiceImpl { system }
    }

    fn cpu_idle(&self) -> Option<f32> {
        match self.system.cpu_load_aggregate() {
            Ok(cpu) => {
                thread::sleep(Duration::from_secs(1));
                match cpu.done() {
                    Ok(cpu) => Some(cpu.idle),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }
}

#[tonic::async_trait]
impl HawkeyeService for HawkeyeServiceImpl {
    async fn get_cpu_stats(
        &self,
        request: Request<CpuStatsRequest>,
    ) -> Result<Response<CpuStatsResponse>, Status> {
        let cpu_stats_response = CpuStatsResponse {
            idle: self.cpu_idle().unwrap(),
        };
        Ok(Response::new(cpu_stats_response))
    }
}