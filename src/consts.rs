use std::{net::SocketAddr, sync::LazyLock};

pub const HOST: LazyLock<SocketAddr> = LazyLock::new(|| "127.0.0.1:4433".parse().unwrap());
