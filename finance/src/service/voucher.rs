use crate::entity::{voucher, voucher_item};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set, ActiveModelTrait, ColumnTrait, TransactionTrait};
use sea_orm::prelude::Decimal;
use uuid::Uuid;
use chrono::Utc;
use super::{ServiceError, ServiceResult};

pub struct VoucherService {
    db: DatabaseConnection,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateVoucherRequest {
    pub tenant_id: Uuid,
    pub org_id: Uuid,
    pub voucher_date: chrono::NaiveDate,
    pub voucher_type: String,
    pub description: Option<String>,
    pub items: Vec<CreateVoucherItemRequest>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateVoucherItemRequest {
    pub account_id: Uuid,
    pub description: Option<String>,
    pub debit_amount: Decimal,
    pub credit_amount: Decimal,
}

impl VoucherService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
    ) -> ServiceResult<Vec<voucher::Model>> {
        let vouchers = voucher::Entity::find()
            .filter(voucher::Column::TenantId.eq(tenant_id))
            .filter(voucher::Column::OrgId.eq(org_id))
            .order_by_desc(voucher::Column::VoucherDate)
            .all(&self.db)
            .await?;

        Ok(vouchers)
    }

    pub async fn get_by_id(&self, id: Uuid) -> ServiceResult<voucher::Model> {
        let voucher = voucher::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| ServiceError::NotFound(format!("Voucher not found: {}", id)))?;

        Ok(voucher)
    }

    pub async fn get_items(&self, voucher_id: Uuid) -> ServiceResult<Vec<voucher_item::Model>> {
        let items = voucher_item::Entity::find()
            .filter(voucher_item::Column::VoucherId.eq(voucher_id))
            .all(&self.db)
            .await?;

        Ok(items)
    }

    pub async fn create(&self, req: CreateVoucherRequest) -> ServiceResult<voucher::Model> {
        // 验证借贷平衡
        let total_debit: Decimal = req.items.iter()
            .map(|item| item.debit_amount)
            .sum();
        let total_credit: Decimal = req.items.iter()
            .map(|item| item.credit_amount)
            .sum();

        if total_debit != total_credit {
            return Err(ServiceError::ValidationError("Debit and credit amounts must be balanced".to_string()));
        }

        // 生成凭证号
        let voucher_no = format!("V{}{:04}", Utc::now().format("%Y%m%d"), rand::random::<u16>() % 10000);

        // 使用事务
        let txn = self.db.begin().await.map_err(|e| ServiceError::DatabaseError(e))?;

        // 创建凭证
        let voucher_id = Uuid::new_v4();
        let voucher_model = voucher::ActiveModel {
            id: Set(voucher_id),
            tenant_id: Set(req.tenant_id),
            org_id: Set(req.org_id),
            voucher_no: Set(voucher_no),
            voucher_date: Set(req.voucher_date),
            voucher_type: Set(req.voucher_type),
            description: Set(req.description),
            total_debit: Set(total_debit),
            total_credit: Set(total_credit),
            status: Set("draft".to_string()),
            created_by: Set(None),
            approved_by: Set(None),
            approved_at: Set(None),
            posted_by: Set(None),
            posted_at: Set(None),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };
        
        voucher_model.insert(&txn).await.map_err(|e| ServiceError::DatabaseError(e))?;

        // 创建凭证明细
        for item in req.items {
            let item_model = voucher_item::ActiveModel {
                id: Set(Uuid::new_v4()),
                tenant_id: Set(req.tenant_id),
                org_id: Set(req.org_id),
                voucher_id: Set(voucher_id),
                account_id: Set(item.account_id),
                description: Set(item.description),
                debit_amount: Set(item.debit_amount),
                credit_amount: Set(item.credit_amount),
                currency: Set("CNY".to_string()),
                exchange_rate: Set(Decimal::ONE),
                created_at: Set(Utc::now()),
                updated_at: Set(Utc::now()),
            };
            item_model.insert(&txn).await.map_err(|e| ServiceError::DatabaseError(e))?;
        }

        txn.commit().await.map_err(|e| ServiceError::DatabaseError(e))?;

        // 重新查询返回
        self.get_by_id(voucher_id).await
    }

    pub async fn approve(&self, id: Uuid, approved_by: Uuid) -> ServiceResult<voucher::Model> {
        let existing = self.get_by_id(id).await?;

        if existing.status != "submitted" {
            return Err(ServiceError::BusinessError("Voucher must be submitted before approval".to_string()));
        }

        let mut voucher: voucher::ActiveModel = existing.into();
        voucher.status = Set("approved".to_string());
        voucher.approved_by = Set(Some(approved_by));
        voucher.approved_at = Set(Some(Utc::now()));
        voucher.updated_at = Set(Utc::now());

        voucher.update(&self.db).await.map_err(|e| ServiceError::DatabaseError(e))?;
        
        self.get_by_id(id).await
    }

    pub async fn delete(&self, id: Uuid) -> ServiceResult<()> {
        let voucher = self.get_by_id(id).await?;

        if voucher.status != "draft" {
            return Err(ServiceError::BusinessError("Can only delete draft vouchers".to_string()));
        }

        // 删除凭证明细
        voucher_item::Entity::delete_many()
            .filter(voucher_item::Column::VoucherId.eq(id))
            .exec(&self.db)
            .await.map_err(|e| ServiceError::DatabaseError(e))?;

        // 删除凭证
        voucher::Entity::delete_by_id(id)
            .exec(&self.db)
            .await.map_err(|e| ServiceError::DatabaseError(e))?;

        Ok(())
    }
}
