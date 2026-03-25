use sea_orm::{Database, Statement, DatabaseBackend, ConnectionTrait};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::connect("postgres://postgres:123123@localhost:5432/icloud").await?;
    
    let sql = r#"
        UPDATE tenants 
        SET config = '{"registry_url": "https://localhost:30500"}'::jsonb,
            updated_at = NOW()
        WHERE slug = 'default'
        RETURNING id, name, config
    "#;
    
    db.execute(Statement::from_string(DatabaseBackend::Postgres, sql))
        .await?;
    
    println!("Updated tenant registry_url to https://localhost:30500");
    
    Ok(())
}