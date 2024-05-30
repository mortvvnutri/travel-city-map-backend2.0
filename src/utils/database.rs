use tokio_postgres::{Client, NoTls, Error};

pub async fn create_tables(client: &mut Client) -> Result<(), Error> {
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS Users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(50) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            email VARCHAR(100) UNIQUE NOT NULL,
            yandex_id VARCHAR(100) UNIQUE,
            registration_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    ")?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS Categories (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            parent_id INTEGER REFERENCES Categories(id)
        );
    ")?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS Places (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            description TEXT,
            category_id INTEGER REFERENCES Categories(id),
            latitude DOUBLE PRECISION NOT NULL,
            longitude DOUBLE PRECISION NOT NULL,
            puskinskaya_card BOOLEAN DEFAULT FALSE
        );
    ")?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS Routes (
            id SERIAL PRIMARY KEY,
            user_id INTEGER REFERENCES Users(id),
            name VARCHAR(255),
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            public_transport_info TEXT
        );
    ")?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS RoutePoints (
            id SERIAL PRIMARY KEY,
            user_id INTEGER REFERENCES Users(id),
            route_id INTEGER REFERENCES Routes(id) ON DELETE CASCADE,
            place_id INTEGER REFERENCES Places(id),
            sequence_number INTEGER NOT NULL
        );
    ")?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS Reviews (
            id SERIAL PRIMARY KEY,
            user_id INTEGER REFERENCES Users(id),
            place_id INTEGER REFERENCES Places(id),
            rating INTEGER,
            comment TEXT,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    ")?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS Notifications (
            id SERIAL PRIMARY KEY,
            user_id INTEGER REFERENCES Users(id),
            message TEXT NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            is_read BOOLEAN DEFAULT FALSE
        );
    ")?;

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
