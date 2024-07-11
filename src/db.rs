use sqlx::Row;
use uuid::Uuid;

pub async fn count_orders_per_status(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let rows = sqlx::query("SELECT status, COUNT(*) FROM orders GROUP BY status").fetch_all(pool).await?;

    for row in rows {
        let status: String = row.get(0);
        let count: i64 = row.get(1);
        println!("Status: {}, Count: {}", status, count);
    }

    Ok(())
}

pub async fn top_customers(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let rows = sqlx::query("SELECT user_id, COUNT(*) as order_count FROM orders GROUP BY user_id ORDER BY order_count DESC LIMIT 10").fetch_all(pool).await?;

    for row in rows {
        let user_id: Uuid = row.get(0);
        let order_count: i64 = row.get(1);
        println!("User ID: {}, Order Count: {}", user_id, order_count);
    }

    Ok(())
}

pub async fn total_customers(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let row = sqlx::query("SELECT COUNT(DISTINCT user_id) FROM orders").fetch_one(pool).await?;

    let total_customers: i64 = row.get(0);
    println!("Total number of customers: {}", total_customers);

    Ok(())
}

pub async fn customer_with_most_closed_orders(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let row = sqlx::query("SELECT user_id, COUNT(*) as order_count FROM orders WHERE status = 'CLOSED' GROUP BY user_id ORDER BY order_count DESC LIMIT 1").fetch_one(pool).await?;

    let user_id: Uuid = row.get(0);
    let order_count: i64 = row.get(1);
    println!("User ID with most 'CLOSED' orders: {}, Order Count: {}", user_id, order_count);

    Ok(())
}