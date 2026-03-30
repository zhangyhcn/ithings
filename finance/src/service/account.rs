use crate::entity::account;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set, ActiveModelTrait, ColumnTrait};
use sea_orm::prelude::Decimal;
use uuid::Uuid;
use super::{ServiceError, ServiceResult};

pub struct AccountService {
    db: DatabaseConnection,
}

impl AccountService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
    ) -> ServiceResult<Vec<account::Model>> {
        let accounts = account::Entity::find()
            .filter(account::Column::TenantId.eq(tenant_id))
            .filter(account::Column::OrgId.eq(org_id))
            .order_by_asc(account::Column::AccountCode)
            .all(&self.db)
            .await.map_err(|e| ServiceError::DatabaseError(e))?;

        Ok(accounts)
    }

    pub async fn get_by_id(&self, id: Uuid) -> ServiceResult<account::Model> {
        let account = account::Entity::find_by_id(id)
            .one(&self.db)
            .await.map_err(|e| ServiceError::DatabaseError(e))?
            .ok_or_else(|| ServiceError::NotFound(format!("Account not found: {}", id)))?;

        Ok(account)
    }

    pub async fn create(&self, mut data: account::ActiveModel) -> ServiceResult<account::Model> {
        // 验证科目代码是否重复
        let code = data.account_code.as_ref().clone();
        let tenant_id = data.tenant_id.as_ref().clone();
        let org_id = data.org_id.as_ref().clone();

        let existing = account::Entity::find()
            .filter(account::Column::TenantId.eq(tenant_id))
            .filter(account::Column::OrgId.eq(org_id))
            .filter(account::Column::AccountCode.eq(&code))
            .one(&self.db)
            .await.map_err(|e| ServiceError::DatabaseError(e))?;

        if existing.is_some() {
            return Err(ServiceError::ValidationError(format!("Account code already exists: {}", code)));
        }

        let account_id = data.id.clone().unwrap();
        data.insert(&self.db).await.map_err(|e| ServiceError::DatabaseError(e))?;
        
        // 重新查询返回
        self.get_by_id(account_id).await
    }

    pub async fn update(&self, id: Uuid, mut data: account::ActiveModel) -> ServiceResult<account::Model> {
        let existing = self.get_by_id(id).await?;
        
        data.id = Set(existing.id);
        data.tenant_id = Set(existing.tenant_id);
        data.org_id = Set(existing.org_id);
        data.created_at = Set(existing.created_at);
        
        data.update(&self.db).await.map_err(|e| ServiceError::DatabaseError(e))?;
        
        self.get_by_id(id).await
    }

    pub async fn delete(&self, id: Uuid) -> ServiceResult<()> {
        let account = self.get_by_id(id).await?;
        
        // 检查是否有子科目
        let children = account::Entity::find()
            .filter(account::Column::ParentId.eq(id))
            .one(&self.db)
            .await.map_err(|e| ServiceError::DatabaseError(e))?;

        if children.is_some() {
            return Err(ServiceError::BusinessError("Cannot delete account with children".to_string()));
        }

        account::Entity::delete_by_id(id)
            .exec(&self.db)
            .await.map_err(|e| ServiceError::DatabaseError(e))?;

        Ok(())
    }
}
