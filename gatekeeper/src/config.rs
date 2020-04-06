use std::fs::File;
use std::path::Path;
use std::time::Duration;

use crate::error::{Error, ErrorKind};
use model::{ConnectRule, IpAddr, Ipv4Addr, SocketAddr};

use failure::ResultExt;
use serde_yaml;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub server_ip: IpAddr,
    pub server_port: u16,
    pub conn_rule: ConnectRule,
    pub read_timeout: Option<Duration>,
    pub write_timeout: Option<Duration>,
}

impl ServerConfig {
    pub fn new(server_ip: IpAddr, server_port: u16, conn_rule: ConnectRule) -> Self {
        Self {
            server_ip,
            server_port,
            conn_rule,
            ..Self::default()
        }
    }

    pub fn with_file(server_ip: IpAddr, server_port: u16, rulefile: &Path) -> Result<Self, Error> {
        let path = File::open(rulefile)?;
        let conn_rule = serde_yaml::from_reader(path).context(ErrorKind::Config)?;
        Ok(ServerConfig {
            server_ip,
            server_port,
            conn_rule,
            ..Self::default()
        })
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            server_ip: Ipv4Addr::new(0, 0, 0, 0).into(),
            server_port: 1080,
            conn_rule: ConnectRule::any(),
            read_timeout: Some(Duration::from_millis(2000)),
            write_timeout: Some(Duration::from_millis(2000)),
        }
    }
}

impl ServerConfig {
    pub fn server_addr(&self) -> SocketAddr {
        SocketAddr::new(self.server_ip, self.server_port)
    }
    pub fn connect_rule(&self) -> ConnectRule {
        self.conn_rule.clone()
    }

    pub fn set_read_timeout(&mut self, dur: Option<Duration>) -> &mut Self {
        self.read_timeout = dur;
        self
    }
    pub fn set_write_timeout(&mut self, dur: Option<Duration>) -> &mut Self {
        self.write_timeout = dur;
        self
    }
}
