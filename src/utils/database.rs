use tokio_postgres::{Client, NoTls, Error};

pub async fn setup_database(client: &tokio_postgres::Client) -> Result<(), tokio_postgres::Error> {
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS Users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(50) UNIQUE NOT NULL
        );
    ").await?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS Categories (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            parent_id INTEGER REFERENCES Categories(id)
        );
    ").await?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS Places (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            category_id INTEGER REFERENCES Categories(id)
        );
    ").await?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS Routes (
            id SERIAL PRIMARY KEY,
            user_id INTEGER REFERENCES Users(id)
        );
    ").await?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS RoutePoints (
            id SERIAL PRIMARY KEY,
            route_id INTEGER REFERENCES Routes(id),
            place_id INTEGER REFERENCES Places(id)
        );
    ").await?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS Reviews (
            id SERIAL PRIMARY KEY,
            user_id INTEGER REFERENCES Users(id),
            place_id INTEGER REFERENCES Places(id),
            review_text TEXT NOT NULL,
            rating INTEGER NOT NULL
        );
    ").await?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS Notifications (
            id SERIAL PRIMARY KEY,
            user_id INTEGER REFERENCES Users(id),
            message TEXT NOT NULL,
            read BOOLEAN NOT NULL DEFAULT FALSE
        );
    ").await?;

    Ok(())
}

pub async fn establish_connection() -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect("host=localhost user=your_user password=your_password dbname=your_db", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}
