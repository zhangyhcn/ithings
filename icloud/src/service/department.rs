use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    entity::department,
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct CreateDepartmentRequest {
    pub organization_id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDepartmentRequest {
    pub organization_id: Option<Uuid>,
    pub name: Option<String>,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct DepartmentResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub organization_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<DepartmentResponse>,
}

impl From<department::Model> for DepartmentResponse {
    fn from(model: department::Model) -> Self {
        Self {
            id: model.id,
            tenant_id: model.tenant_id,
            organization_id: model.organization_id,
            parent_id: model.parent_id,
            name: model.name,
            slug: model.slug,
            description: model.description,
            status: model.status,
            created_at: model.created_at,
            updated_at: model.updated_at,
            children: Vec::new(),
        }
    }
}

pub struct DepartmentService {
    db: DatabaseConnection,
}

impl DepartmentService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, org_id: Uuid, mut req: CreateDepartmentRequest) -> Result<DepartmentResponse, AppError> {
        // 生成slug，这里简化为名称的小写拼音或直接使用名称，实际项目中可以使用slugify库
        let slug = req.name.to_lowercase().replace(" ", "-");
        req.organization_id = org_id;

        let dept = department::ActiveModel {
            tenant_id: Set(Uuid::parse_str("24600c16-dba7-449e-a23b-935e1ec8d5d8").unwrap()), // 默认租户ID，实际应该从JWT中获取
            organization_id: Set(req.organization_id),
            parent_id: Set(req.parent_id),
            name: Set(req.name),
            slug: Set(slug),
            description: Set(req.description),
            status: Set(req.status.unwrap_or("active".to_string())),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        let dept = dept.insert(&self.db).await?;
        Ok(DepartmentResponse::from(dept))
    }

    pub async fn find_by_id(&self, org_id: Uuid, id: Uuid) -> Result<DepartmentResponse, AppError> {
        let dept = department::Entity::find_by_id(id)
            .filter(department::Column::OrganizationId.eq(org_id))
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Department not found".to_string()))?;
        Ok(DepartmentResponse::from(dept))
    }

    pub async fn list(&self, org_id: Uuid) -> Result<Vec<DepartmentResponse>, AppError> {
        let depts = department::Entity::find()
            .filter(department::Column::OrganizationId.eq(org_id))
            .all(&self.db)
            .await?;
        
        let mut dept_responses: Vec<DepartmentResponse> = depts.into_iter().map(DepartmentResponse::from).collect();
        
        // 构建树形结构
        let mut tree = Vec::new();
        let mut children_map = std::collections::HashMap::new();
        
        for dept in dept_responses.drain(..) {
            children_map.entry(dept.parent_id).or_insert_with(Vec::new).push(dept);
        }
        
        // 从顶层节点开始构建
        let mut stack = children_map.remove(&None).unwrap_or_default();
        
        while let Some(mut dept) = stack.pop() {
            if let Some(mut children) = children_map.remove(&Some(dept.id)) {
                for child in children.into_iter().rev() {
                    stack.push(child);
                }
                dept.children = children_map.remove(&Some(dept.id)).unwrap_or_default();
            }
            tree.push(dept);
        }
        
        tree.reverse();
        Ok(tree)
    }

    pub async fn update(&self, org_id: Uuid, id: Uuid, mut req: UpdateDepartmentRequest) -> Result<DepartmentResponse, AppError> {
        let dept = department::Entity::find_by_id(id)
            .filter(department::Column::OrganizationId.eq(org_id))
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Department not found".to_string()))?;
        
        // 不允许修改organization_id
        req.organization_id = None;

        let mut dept: department::ActiveModel = dept.into();

        if let Some(organization_id) = req.organization_id {
            dept.organization_id = Set(organization_id);
        }
        if let Some(name) = req.name {
            dept.name = Set(name.clone());
            dept.slug = Set(name.to_lowercase().replace(" ", "-"));
        }
        if let Some(parent_id) = req.parent_id {
            dept.parent_id = Set(Some(parent_id));
        }
        if let Some(description) = req.description {
            dept.description = Set(Some(description));
        }
        if let Some(status) = req.status {
            dept.status = Set(status);
        }
        dept.updated_at = Set(Utc::now().naive_utc());

        let dept = dept.update(&self.db).await?;
        Ok(DepartmentResponse::from(dept))
    }

    pub async fn delete(&self, org_id: Uuid, id: Uuid) -> Result<(), AppError> {
        // 验证部门属于指定的组织
        let dept = department::Entity::find_by_id(id)
            .filter(department::Column::OrganizationId.eq(org_id))
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Department not found".to_string()))?;
        
        // 先查询所有子部门ID
        let mut all_ids = vec![id];
        let mut current_level = vec![id];
        
        while !current_level.is_empty() {
            let mut next_level = Vec::new();
            for parent_id in current_level {
                let children = department::Entity::find()
                    .filter(department::Column::ParentId.eq(parent_id))
                    .all(&self.db)
                    .await?;
                
                for child in children {
                    all_ids.push(child.id);
                    next_level.push(child.id);
                }
            }
            current_level = next_level;
        }

        // 从最底层开始删除
        all_ids.reverse();
        for dept_id in all_ids {
            let result = department::Entity::delete_by_id(dept_id)
                .exec(&self.db)
                .await?;

            if result.rows_affected == 0 {
                return Err(AppError::NotFound("Department not found".to_string()));
            }
        }

        Ok(())
    }
}
