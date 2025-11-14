use zbus::{connection::Builder, interface, Connection, Result};

pub struct BootloaderConfig {}

#[interface(name = "org.opensuse.bootloader.Config")]
impl BootloaderConfig {
    async fn get_config(&self) -> String {
        String::from("Hello world!")
    }
}

pub async fn create_connection() -> Result<Connection> {
    let config = BootloaderConfig {};

    let connection = Builder::session()?
        .name("org.opensuse.bootloader.Config")?
        .serve_at("/org/opensuse/bootloader", config)?
        .build()
        .await?;

    Ok(connection)
}
