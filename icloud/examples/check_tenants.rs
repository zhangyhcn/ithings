use sea_orm::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::connect("postgres://postgres:123123@localhost:5432/icloud").await?;
    
    let tenants = icloud::entity::TenantEntity::find().all(&db).await?;
    
    println!("Tenants in database:");
    for tenant in tenants {
        println!("ID: {}, Name: {}, Slug: {}, Config: {:?}", 
                 tenant.id, tenant.name, tenant.slug, tenant.config);
    }
    
    Ok(())
}