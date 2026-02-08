use bollard::container::{Config, CreateContainerOptions, StartContainerOptions, RemoveContainerOptions};
use bollard::Docker;
use std::collections::HashMap;
use std::error::Error;

pub struct Orchestrator {
    docker: Docker,
}

impl Orchestrator {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let docker = Docker::connect_with_local_defaults()?;
        Ok(Self { docker })
    }

    pub async fn create_mission_container(&self, image: &str, mission_id: &str) -> Result<String, Box<dyn Error>> {
        let container_name = format!("cyberlab-{}", mission_id);
        
        let config = Config {
            image: Some(image),
            tty: Some(true),
            open_stdin: Some(true),
            host_config: Some(bollard::service::HostConfig {
                network_mode: Some("none".to_string()), // Start with no network for security
                memory: Some(512 * 1024 * 1024), // 512MB
                cpu_quota: Some(50000), // 50% of one core
                ..Default::default()
            }),
            ..Default::default()
        };

        let options = Some(CreateContainerOptions {
            name: container_name.clone(),
            ..Default::default()
        });

        self.docker.create_container(options, config).await?;
        self.docker.start_container(&container_name, None::<StartContainerOptions<String>>).await?;

        Ok(container_name)
    }

    pub async fn cleanup_mission(&self, container_name: &str) -> Result<(), Box<dyn Error>> {
        let options = Some(RemoveContainerOptions {
            force: true,
            ..Default::default()
        });
        self.docker.remove_container(container_name, options).await?;
        Ok(())
    }
}
