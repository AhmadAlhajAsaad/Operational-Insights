//! Fix inconsistent Atlassian links
//! Reset persons with link status but no account_id to 'unlinked'

use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load database URL from environment
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://equans:equans_password@postgres/equans_insights".to_string()
    });

    println!("Connecting to database...");
    let pool = PgPool::connect(&database_url).await?;

    // Count affected records
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM persons
         WHERE atlassian_link_status NOT IN ('unlinked', 'no_atlassian_account')
         AND atlassian_account_id IS NULL",
    )
    .fetch_one(&pool)
    .await?;

    println!("Found {} persons with inconsistent link status", count.0);

    if count.0 == 0 {
        println!("No inconsistent links found. Exiting.");
        return Ok(());
    }

    // Reset inconsistent links
    let result = sqlx::query(
        "UPDATE persons
         SET atlassian_link_status = 'unlinked',
             atlassian_linked_at = NULL,
             atlassian_link_method = NULL
         WHERE atlassian_link_status NOT IN ('unlinked', 'no_atlassian_account')
         AND atlassian_account_id IS NULL",
    )
    .execute(&pool)
    .await?;

    println!(
        "✅ Reset {} persons to 'unlinked' status",
        result.rows_affected()
    );

    // Count unlinked records now
    let unlinked: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM persons
         WHERE atlassian_link_status = 'unlinked'
         AND atlassian_account_id IS NULL",
    )
    .fetch_one(&pool)
    .await?;

    println!("Total unlinked persons: {}", unlinked.0);
    println!("\n✅ Fix complete!");
    println!("Now trigger relink via API:");
    println!("curl -X POST -H 'Content-Type: application/json' -d '{{}}' http://localhost:8080/api/atlassian/link-persons");

    Ok(())
}
