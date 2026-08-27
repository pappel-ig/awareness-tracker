use anyhow::Result;
use tokio_postgres::{Client, NoTls};

pub async fn connect(database_url: &str) -> Result<Client> {
    let (client, connection) = tokio_postgres::connect(database_url, NoTls).await?;
    tokio::spawn(async move {
        let _ = connection.await;
    });

    client
        .batch_execute(
            "CREATE TABLE IF NOT EXISTS tracks (
                id UUID PRIMARY KEY,
                created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                method TEXT NOT NULL,
                uri TEXT NOT NULL,
                remote_addr TEXT NOT NULL,
                tls JSONB NOT NULL,
                headers JSONB NOT NULL
            )",
        )
        .await?;

    Ok(client)
}
