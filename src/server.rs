use std::{thread, time};
use crate::antd::ant_daemon_server::{AntDaemon, AntDaemonServer};
use crate::antd::{HeartRateMessage, ListSensorsRequest, ListSensorsResponse, Sensor, SensorType, StreamHeartRateRequest};
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;

#[derive(Debug, Default)]
pub struct VotingService {}

#[derive(Debug, Default)]
pub struct AntDaemonService {}

#[tonic::async_trait]
impl AntDaemon for AntDaemonService {
    async fn list_sensors(&self, request: Request<ListSensorsRequest>) -> Result<Response<ListSensorsResponse>, Status> {

        let mut sensors = Vec::new();
        sensors.push(Sensor{
            device_id: 123,
            serial_number: 1234,
            sensor_type: SensorType::HeartRate as i32,
        });

        Ok(Response::new(ListSensorsResponse{
            sensors
        }))
    }

    type StreamHeartRateStream = ReceiverStream<Result<HeartRateMessage, Status>>;

    async fn stream_heart_rate(&self, request: Request<StreamHeartRateRequest>) -> Result<Response<Self::StreamHeartRateStream>, Status> {

        let (tx, rx) = mpsc::channel(1);

        let mut heart_rate_messages = Vec::new();
        heart_rate_messages.push(HeartRateMessage{
            heart_rate: 132,
        });
        heart_rate_messages.push(HeartRateMessage{
            heart_rate: 184,
        });
        heart_rate_messages.push(HeartRateMessage{
            heart_rate: 159,
        });
        tokio::spawn(async move {
            for hr_msg in &heart_rate_messages {
                tx.send(Ok(hr_msg.clone())).await.unwrap();
                thread::sleep(time::Duration::from_millis(2000));
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:8080".parse().unwrap();
    let antd_service = AntDaemonService::default();

    Server::builder().add_service(AntDaemonServer::new(antd_service))
        .serve(address)
        .await?;
    Ok(())
}