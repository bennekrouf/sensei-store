use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub grpc: GrpcServerConfig,
    pub http: HttpServerConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GrpcServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    // pub whoami: String,
    // pub output: String,
    // pub level: String,
    pub server: ServerConfig,
    // Include other fields as needed
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    pub fn grpc_address(&self) -> String {
        format!("{}:{}", self.server.grpc.host, self.server.grpc.port)
    }

    pub fn http_host(&self) -> &str {
        &self.server.http.host
    }

    pub fn http_port(&self) -> u16 {
        self.server.http.port
    }
}

// Default implementation for testing or when config file is missing
impl Default for Config {
    fn default() -> Self {
        Self {
            // whoami: "api-store".to_string(),
            // output: "grpc".to_string(),
            // level: "debug".to_string(),
            server: ServerConfig {
                grpc: GrpcServerConfig {
                    host: "0.0.0.0".to_string(),
                    port: 50055,
                },
                http: HttpServerConfig {
                    host: "127.0.0.1".to_string(),
                    port: 9090,
                },
            },
        }
    }
}
