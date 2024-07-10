mod db;

use chrono::{DateTime, Utc};
use rand::Rng;
use sqlx::postgres::PgPoolOptions;
use std::env;
use uuid::Uuid;

#[derive(Debug)]
struct Order {
    id: i32,
    timestamp: DateTime<Utc>,
    user_id: Uuid,
    status: String,
}

fn generate_random_order(id: i32) -> Order {
    let statuses = [
        "CLOSED", "CANCELED", "COMPLETE", "PENDING_PAYMENT", "SUSPECTED_FRAUD",
        "PENDING", "ON_HOLD", "PROCESSING", "PAYMENT_REVIEW",
    ];
    let status = statuses[rand::thread_rng().gen_range(0..statuses.len())];
    Order {
        id,
        timestamp: Utc::now(),
        user_id: Uuid::new_v4(),
        status: status.to_string(),
    }
}

async fn generate_and_insert_orders(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    for id in 1..=100000 {
        let order = generate_random_order(id);
        sqlx::query(
            "INSERT INTO orders (id, timestamp, user_id, status) VALUES ($1, $2, $3, $4)",
        )
        .bind(order.id)
        .bind(order.timestamp)
        .bind(order.user_id)
        .bind(&order.status)
        .execute(pool)
        .await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Create the table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS orders (
            id SERIAL PRIMARY KEY,
            timestamp TIMESTAMPTZ NOT NULL,
            user_id UUID NOT NULL,
            status TEXT NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // Generate and insert 100k orders
    generate_and_insert_orders(&pool).await?;

    // Perform the required queries
    db::count_orders_per_status(&pool).await?;
    db::top_customers(&pool).await?;
    db::total_customers(&pool).await?;
    db::customer_with_most_closed_orders(&pool).await?;

    Ok(())
}