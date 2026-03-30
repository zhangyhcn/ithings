use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait, Set, QuerySelect};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;

use crate::entity::{MaterialEntity, MaterialColumn, MaterialModel as Model};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMaterialRequest {
    pub category_id: Option<String>,
    pub code: String,
    pub name: String,
    pub specification: Option<String>,
    pub model: Option<String>,
    pub unit: String,
    pub unit_weight: Option<f64>,
    pub unit_volume: Option<f64>,
    pub barcode: Option<String>,
    pub safety_stock: Option<f64>,
    pub max_stock: Option<f64>,
    pub min_order_qty: Option<f64>,
    pub lead_time: Option<i32>,
    pub purchase_price: Option<f64>,
    pub sale_price: Option<f64>,
    pub cost_price: Option<f64>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMaterialRequest {
    pub category_id: Option<String>,
    pub name: Option<String>,
    pub specification: Option<String>,
    pub model: Option<String>,
    pub unit: Option<String>,
    pub unit_weight: Option<f64>,
    pub unit_volume: Option<f64>,
    pub barcode: Option<String>,
    pub status: Option<String>,
    pub safety_stock: Option<f64>,
    pub max_stock: Option<f64>,
    pub min_order_qty: Option<f64>,
    pub lead_time: Option<i32>,
    pub purchase_price: Option<f64>,
    pub sale_price: Option<f64>,
    pub cost_price: Option<f64>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialResponse {
    pub id: String,
    pub tenant_id: String,
    pub org_id: String,
    pub category_id: Option<String>,
    pub code: String,
    pub name: String,
    pub specification: Option<String>,
    pub model: Option<String>,
    pub unit: String,
    pub unit_weight: Option<f64>,
    pub unit_volume: Option<f64>,
    pub barcode: Option<String>,
    pub status: String,
    pub safety_stock: f64,
    pub max_stock: f64,
    pub min_order_qty: f64,
    pub lead_time: i32,
    pub purchase_price: Option<f64>,
    pub sale_price: Option<f64>,
    pub cost_price: Option<f64>,
    pub remark: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for MaterialResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            org_id: model.org_id.to_string(),
            category_id: model.category_id.map(|id| id.to_string()),
            code: model.code,
            name: model.name,
            specification: model.specification,
            model: model.model,
            unit: model.unit,
            unit_weight: model.unit_weight.map(|d| d.to_string().parse().unwrap()),
            unit_volume: model.unit_volume.map(|d| d.to_string().parse().unwrap()),
            barcode: model.barcode,
            status: model.status,
            safety_stock: model.safety_stock.to_string().parse().unwrap(),
            max_stock: model.max_stock.to_string().parse().unwrap(),
            min_order_qty: model.min_order_qty.to_string().parse().unwrap(),
            lead_time: model.lead_time,
            purchase_price: model.purchase_price.map(|d| d.to_string().parse().unwrap()),
            sale_price: model.sale_price.map(|d| d.to_string().parse().unwrap()),
            cost_price: model.cost_price.map(|d| d.to_string().parse().unwrap()),
            remark: model.remark,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct MaterialService {
    db: DatabaseConnection,
}

impl MaterialService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
        req: CreateMaterialRequest,
    ) -> Result<MaterialResponse, AppError> {
        let id = Uuid::new_v4();
        let now = Utc::now().naive_utc();
        
        let material = crate::entity::material::ActiveModel {
            id: Set(id),
            tenant_id: Set(tenant_id),
            org_id: Set(org_id),
            category_id: Set(req.category_id.and_then(|s| Uuid::parse_str(&s).ok())),
            code: Set(req.code),
            name: Set(req.name),
            specification: Set(req.specification),
            model: Set(req.model),
            unit: Set(req.unit),
            unit_weight: Set(req.unit_weight.map(|v| Decimal::from_f64_retain(v).unwrap_or_default())),
            unit_volume: Set(req.unit_volume.map(|v| Decimal::from_f64_retain(v).unwrap_or_default())),
            barcode: Set(req.barcode),
            status: Set("active".to_string()),
            safety_stock: Set(Decimal::from_f64_retain(req.safety_stock.unwrap_or(0.0)).unwrap_or_default()),
            max_stock: Set(Decimal::from_f64_retain(req.max_stock.unwrap_or(0.0)).unwrap_or_default()),
            min_order_qty: Set(Decimal::from_f64_retain(req.min_order_qty.unwrap_or(0.0)).unwrap_or_default()),
            lead_time: Set(req.lead_time.unwrap_or(0)),
            purchase_price: Set(req.purchase_price.map(|v| Decimal::from_f64_retain(v).unwrap_or_default())),
            sale_price: Set(req.sale_price.map(|v| Decimal::from_f64_retain(v).unwrap_or_default())),
            cost_price: Set(req.cost_price.map(|v| Decimal::from_f64_retain(v).unwrap_or_default())),
            remark: Set(req.remark),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let material = material.insert(&self.db).await?;
        Ok(material.into())
    }

    pub async fn list_all(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
    ) -> Result<Vec<MaterialResponse>, AppError> {
        let materials = MaterialEntity::find()
            .filter(MaterialColumn::TenantId.eq(tenant_id))
            .filter(MaterialColumn::OrgId.eq(org_id))
            .all(&self.db)
            .await?;

        Ok(materials.into_iter().map(|m| m.into()).collect())
    }

    pub async fn get_by_id(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<Option<MaterialResponse>, AppError> {
        let material = MaterialEntity::find()
            .filter(MaterialColumn::TenantId.eq(tenant_id))
            .filter(MaterialColumn::OrgId.eq(org_id))
            .filter(MaterialColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        Ok(material.map(|m| m.into()))
    }

    pub async fn update(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
        id: Uuid,
        req: UpdateMaterialRequest,
    ) -> Result<MaterialResponse, AppError> {
        let material = MaterialEntity::find()
            .filter(MaterialColumn::TenantId.eq(tenant_id))
            .filter(MaterialColumn::OrgId.eq(org_id))
            .filter(MaterialColumn::Id.eq(id))
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::not_found("Material not found".to_string()))?;

        let now = Utc::now().naive_utc();
        let mut material: crate::entity::material::ActiveModel = material.into();
        
        if let Some(category_id) = req.category_id {
            material.category_id = Set(Uuid::parse_str(&category_id).ok());
        }
        if let Some(name) = req.name {
            material.name = Set(name);
        }
        if let Some(specification) = req.specification {
            material.specification = Set(Some(specification));
        }
        if let Some(model) = req.model {
            material.model = Set(Some(model));
        }
        if let Some(unit) = req.unit {
            material.unit = Set(unit);
        }
        if let Some(unit_weight) = req.unit_weight {
            material.unit_weight = Set(Some(Decimal::from_f64_retain(unit_weight).unwrap_or_default()));
        }
        if let Some(unit_volume) = req.unit_volume {
            material.unit_volume = Set(Some(Decimal::from_f64_retain(unit_volume).unwrap_or_default()));
        }
        if let Some(barcode) = req.barcode {
            material.barcode = Set(Some(barcode));
        }
        if let Some(status) = req.status {
            material.status = Set(status);
        }
        if let Some(safety_stock) = req.safety_stock {
            material.safety_stock = Set(Decimal::from_f64_retain(safety_stock).unwrap_or_default());
        }
        if let Some(max_stock) = req.max_stock {
            material.max_stock = Set(Decimal::from_f64_retain(max_stock).unwrap_or_default());
        }
        if let Some(min_order_qty) = req.min_order_qty {
            material.min_order_qty = Set(Decimal::from_f64_retain(min_order_qty).unwrap_or_default());
        }
        if let Some(lead_time) = req.lead_time {
            material.lead_time = Set(lead_time);
        }
        if let Some(purchase_price) = req.purchase_price {
            material.purchase_price = Set(Some(Decimal::from_f64_retain(purchase_price).unwrap_or_default()));
        }
        if let Some(sale_price) = req.sale_price {
            material.sale_price = Set(Some(Decimal::from_f64_retain(sale_price).unwrap_or_default()));
        }
        if let Some(cost_price) = req.cost_price {
            material.cost_price = Set(Some(Decimal::from_f64_retain(cost_price).unwrap_or_default()));
        }
        if let Some(remark) = req.remark {
            material.remark = Set(Some(remark));
        }
        
        material.updated_at = Set(now);

        let material = material.update(&self.db).await?;
        Ok(material.into())
    }

    pub async fn delete(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<(), AppError> {
        MaterialEntity::delete_by_id(id)
            .filter(MaterialColumn::TenantId.eq(tenant_id))
            .filter(MaterialColumn::OrgId.eq(org_id))
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
