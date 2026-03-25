use sea_orm::Database;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::connect("postgres://postgres:123123@localhost:5432/icloud").await?;
    
    let tenants = icloud::entity::TenantEntity::find().all(&db).await?;
    
    println!("Tenants in database:");
    for tenant in tenants {
        println!("ID: {}, Name: {}, Slug: {}, Config: {:?}", 
                 tenant.id, tenant.name, tenant.slug, tenant.config);
        
        if let Some(config) = &tenant.config {
            if let Some(registry_url) = config.get("registry_url") {
                let url = registry_url.as_str().unwrap_or("");
                println!("  Registry URL: {}", url);
                
                if !url.is_empty() {
                    test_registry_connection(url).await;
                }
            } else {
                println!("  No registry_url configured in config");
            }
        }
    }
    
    Ok(())
}

async fn test_registry_connection(registry_url: &str) {
    let client = Client::new();
    let registry = registry_url.trim_end_matches('/');
    
    println!("Testing registry connection to: {}", registry);
    
    let catalog_url = format!("{}/v2/_catalog", registry);
    println!("Catalog URL: {}", catalog_url);
    
    match client.get(&catalog_url).send().await {
        Ok(response) => {
            println!("Response status: {}", response.status());
            if response.status().is_success() {
                match response.text().await {
                    Ok(text) => {
                        println!("Response body: {}", text);
                    }
                    Err(e) => {
                        println!("Failed to read response: {}", e);
                    }
                }
            } else {
                match response.text().await {
                    Ok(text) => {
                        println!("Error response: {}", text);
                    }
                    Err(e) => {
                        println!("Failed to read error response: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect to registry: {}", e);
        }
    }
}