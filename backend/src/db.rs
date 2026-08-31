use std::env;
use anyhow::Result;
use tokio_postgres::{Client, NoTls};

pub async fn connect(database_url: &str) -> Result<Client> {
    let (client, connection) = tokio_postgres::connect(database_url, NoTls).await?;
    tokio::spawn(async move {
        let _ = connection.await;
    });

    if env::var("DEBUG").is_ok() {
        client.batch_execute("
            DROP TABLE IF EXISTS tracks;
                DROP TABLE IF EXISTS participants;
        ").await?;
    }

    client
        .batch_execute(
            "
                CREATE TABLE IF NOT EXISTS tracks (
                    id UUID PRIMARY KEY,
                    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                    method TEXT NOT NULL,
                    uri TEXT NOT NULL,
                    remote_addr TEXT NOT NULL,
                    tls JSONB NOT NULL,
                    headers JSONB NOT NULL
                );

                CREATE TABLE IF NOT EXISTS participants (
                    id UUID PRIMARY KEY,
                    email TEXT UNIQUE NOT NULL,
                    leak_check BOOLEAN,
                    leak_breaches JSONB,
                    registered_at TIMESTAMPTZ NOT NULL DEFAULT now()
                );
            "
        )
        .await?;

    Ok(client)
}
