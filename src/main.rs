use zbus::{proxy, Connection, Result};

#[proxy(
    interface = "org.mechanix.services.Wireless",
    default_service = "org.mechanix.services.Wireless",
    default_path = "/org/mechanix/services/Wireless"
)]
trait Wireless {
    async fn status(&self) -> Result<bool>;
}

// Although we use `async-std` here, you can use any async runtime of choice.
#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;

    // `proxy` macro creates `WirelessProxy` based on `Notifications` trait.
    let proxy = WirelessProxy::new(&connection).await?;
    let reply = proxy.status().await?;
    println!("{:?}", reply);

    Ok(())
}
