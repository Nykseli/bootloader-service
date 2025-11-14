use std::future::pending;
use zbus::Result;

mod dbus;
use crate::dbus::connection::create_connection;

#[tokio::main]
async fn main() -> Result<()> {
    let _connection = create_connection().await?;
    pending::<()>().await;
    Ok(())
}
