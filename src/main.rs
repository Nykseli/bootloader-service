use clap::Parser;
use std::future::pending;
use zbus::Result;

mod config;
mod dbus;
mod grub2;
use crate::{config::ConfigArgs, dbus::connection::create_connection};

#[tokio::main]
async fn main() -> Result<()> {
    let args = ConfigArgs::parse();

    let _connection = create_connection(&args).await?;
    pending::<()>().await;
    Ok(())
}
