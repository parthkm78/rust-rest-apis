use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::{TokioAsyncWriteCompatExt, Compat};
use std::sync::Arc;

pub type DbClient = Arc<tokio::sync::Mutex<Client<Compat<TcpStream>>>>;

pub async fn init_db() -> Result<DbClient, Box<dyn std::error::Error>> {
    let mut config = Config::new();
    
    config.host("192.168.102.120");
    config.port(1433);
    config.database("TEST_1");
    config.authentication(AuthMethod::sql_server("sa", "963852"));
    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let client = Client::connect(config, tcp.compat_write()).await?;
    
    Ok(Arc::new(tokio::sync::Mutex::new(client)))
}
