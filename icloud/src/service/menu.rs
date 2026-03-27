use crate::entity::{menu, user, user_role, role, role_menu};
use sea_orm::*;
use sea_orm::prelude::Json;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuCreateRequest {
    pub parent_id: Option<String>,
    pub name: String,
    pub path: String,
    pub component: String,
    pub icon: Option<String>,
    pub sort_order: i32,
    pub status: String,
    pub roles: Vec<String>,
    pub i18n_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuUpdateRequest {
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub path: Option<String>,
    pub component: Option<String>,
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
    pub status: Option<String>,
    pub roles: Option<Vec<String>>,
    pub i18n_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuTree {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub path: String,
    pub component: String,
    pub icon: Option<String>,
    pub sort_order: i32,
    pub status: String,
    pub roles: Vec<String>,
    pub i18n_key: Option<String>,
    pub children: Option<Vec<MenuTree>>,
}

pub struct MenuService;

impl MenuService {
    pub async fn create(db: &DbConn, req: MenuCreateRequest) -> Result<menu::Model, DbErr> {
        let menu_id = Uuid::new_v4();
        let now = Utc::now().naive_utc();
        let parent_id = req.parent_id.as_ref().map(|s| Uuid::parse_str(s).map_err(|e| DbErr::Custom(e.to_string()))).transpose()?;
        
        let menu = menu::ActiveModel {
            id: Set(menu_id),
            parent_id: Set(parent_id),
            name: Set(req.name),
            path: Set(req.path),
            component: Set(req.component),
            icon: Set(req.icon),
            sort_order: Set(req.sort_order),
            status: Set(req.status),
            roles: Set(serde_json::to_value(req.roles).map_err(|e| DbErr::Custom(e.to_string()))?),
            i18n_key: Set(req.i18n_key),
            created_at: Set(now),
            updated_at: Set(now),
        };
        
        menu.insert(db).await
    }

    pub async fn update(db: &DbConn, id: String, req: MenuUpdateRequest) -> Result<menu::Model, DbErr> {
        let id_uuid = Uuid::parse_str(&id).map_err(|e| DbErr::Custom(e.to_string()))?;
        let mut menu: menu::ActiveModel = menu::Entity::find_by_id(id_uuid)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Menu not found".to_string()))?
            .into();

        let now = Utc::now().naive_utc();

        if let Some(parent_id) = req.parent_id {
            let parent_uuid = Uuid::parse_str(&parent_id).map_err(|e| DbErr::Custom(e.to_string()))?;
            menu.parent_id = Set(Some(parent_uuid));
        }
        if let Some(name) = req.name {
            menu.name = Set(name);
        }
        if let Some(path) = req.path {
            menu.path = Set(path);
        }
        if let Some(component) = req.component {
            menu.component = Set(component);
        }
        if let Some(icon) = req.icon {
            menu.icon = Set(Some(icon));
        }
        if let Some(sort_order) = req.sort_order {
            menu.sort_order = Set(sort_order);
        }
        if let Some(status) = req.status {
            menu.status = Set(status);
        }
        if let Some(roles) = req.roles {
            menu.roles = Set(serde_json::to_value(roles).map_err(|e| DbErr::Custom(e.to_string()))?);
        }
        if let Some(i18n_key) = req.i18n_key {
            menu.i18n_key = Set(Some(i18n_key));
        }
        menu.updated_at = Set(now);

        menu.update(db).await
    }

    pub async fn delete(db: &DbConn, id: String) -> Result<(), DbErr> {
        let id_uuid = Uuid::parse_str(&id).map_err(|e| DbErr::Custom(e.to_string()))?;
        let menu = menu::Entity::find_by_id(id_uuid)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Menu not found".to_string()))?;

        menu.delete(db).await?;
        Ok(())
    }

    pub async fn get_by_id(db: &DbConn, id: String) -> Result<menu::Model, DbErr> {
        let id_uuid = Uuid::parse_str(&id).map_err(|e| DbErr::Custom(e.to_string()))?;
        menu::Entity::find_by_id(id_uuid)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Menu not found".to_string()))
    }

    pub async fn list_all(db: &DbConn) -> Result<Vec<menu::Model>, DbErr> {
        menu::Entity::find()
            .order_by_asc(menu::Column::SortOrder)
            .all(db)
            .await
    }

    pub async fn get_user_menus(db: &DbConn, username: String) -> Result<Vec<MenuTree>, DbErr> {
        // 获取用户信息
        let user = user::Entity::find()
            .filter(user::Column::Username.eq(username))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("User not found".to_string()))?;

        // 获取用户角色
        let user_roles = user_role::Entity::find()
            .filter(user_role::Column::UserId.eq(user.id))
            .all(db)
            .await?;

        let mut role_ids: Vec<Uuid> = user_roles.into_iter().map(|ur| ur.role_id).collect();

        // 如果用户没有角色，默认添加user角色
        if role_ids.is_empty() {
            let user_role = role::Entity::find()
                .filter(role::Column::Name.eq("user"))
                .one(db)
                .await?;
            
            if let Some(user_role) = user_role {
                role_ids.push(user_role.id);
            } else {
                return Ok(vec![]);
            }
        }

        // 获取角色关联的菜单
        let role_menus = role_menu::Entity::find()
            .filter(role_menu::Column::RoleId.is_in(role_ids))
            .all(db)
            .await?;

        let menu_ids: Vec<Uuid> = role_menus.into_iter().map(|rm| rm.menu_id).collect();

        if menu_ids.is_empty() {
            return Ok(vec![]);
        }

        // 获取菜单详情
        let menus = menu::Entity::find()
            .filter(menu::Column::Id.is_in(menu_ids))
            .filter(menu::Column::Status.eq("active"))
            .order_by_asc(menu::Column::SortOrder)
            .all(db)
            .await?;

        // 构建菜单树
        let menu_trees = Self::build_menu_tree(menus, None)?;

        Ok(menu_trees)
    }

    pub async fn get_menu_tree(db: &DbConn) -> Result<Vec<MenuTree>, DbErr> {
        let menus = menu::Entity::find()
            .order_by_asc(menu::Column::SortOrder)
            .all(db)
            .await?;

        let menu_trees = Self::build_menu_tree(menus, None)?;

        Ok(menu_trees)
    }

    fn build_menu_tree(menus: Vec<menu::Model>, parent_id: Option<Uuid>) -> Result<Vec<MenuTree>, DbErr> {
        let mut children = Vec::new();

        for menu in &menus {
            if menu.parent_id == parent_id {
                let mut tree_node = MenuTree {
                    id: menu.id.to_string(),
                    parent_id: menu.parent_id.map(|p| p.to_string()),
                    name: menu.name.clone(),
                    path: menu.path.clone(),
                    component: menu.component.clone(),
                    icon: menu.icon.clone(),
                    sort_order: menu.sort_order,
                    status: menu.status.clone(),
                    roles: serde_json::from_value(menu.roles.clone()).map_err(|e| DbErr::Custom(e.to_string()))?,
                    i18n_key: menu.i18n_key.clone(),
                    children: None,
                };

                let sub_children = Self::build_menu_tree(menus.clone(), Some(menu.id))?;
                if !sub_children.is_empty() {
                    tree_node.children = Some(sub_children);
                }

                children.push(tree_node);
            }
        }

        Ok(children)
    }

    pub async fn init_default_menus(db: &DbConn) -> Result<(), DbErr> {
        let now = Utc::now().naive_utc();
        
        // 定义所有默认菜单
        #[derive(Debug, Clone)]
        struct DefaultMenu {
            name: &'static str,
            path: &'static str,
            component: &'static str,
            icon: Option<&'static str>,
            parent_path: Option<&'static str>,
            sort_order: i32,
            status: &'static str,
            roles: Vec<&'static str>,
            i18n_key: Option<&'static str>,
        }

        let default_menus = vec![
            // 一级菜单
            DefaultMenu {
                name: "仪表盘",
                path: "/dashboard",
                component: "@/pages/Dashboard",
                icon: Some("DashboardOutlined"),
                parent_path: None,
                sort_order: 1,
                status: "active",
                roles: vec!["admin", "editor", "user"],
                i18n_key: Some("menu.dashboard"),
            },
            DefaultMenu {
                name: "系统设置",
                path: "/settings",
                component: "Layout",
                icon: Some("SettingOutlined"),
                parent_path: None,
                sort_order: 2,
                status: "active",
                roles: vec!["admin"],
                i18n_key: Some("menu.setting"),
            },
            DefaultMenu {
                name: "资源管理",
                path: "/resources",
                component: "Layout",
                icon: Some("DatabaseOutlined"),
                parent_path: None,
                sort_order: 3,
                status: "active",
                roles: vec!["admin", "editor"],
                i18n_key: Some("menu.resource"),
            },
            DefaultMenu {
                name: "配置管理",
                path: "/config",
                component: "Layout",
                icon: Some("SettingOutlined"),
                parent_path: None,
                sort_order: 4,
                status: "active",
                roles: vec!["admin", "editor"],
                i18n_key: Some("menu.config"),
            },
            DefaultMenu {
                name: "设备管理",
                path: "/device",
                component: "Layout",
                icon: Some("MobileOutlined"),
                parent_path: None,
                sort_order: 5,
                status: "active",
                roles: vec!["admin", "editor"],
                i18n_key: Some("menu.device"),
            },
            // 系统设置二级菜单
            DefaultMenu {
                name: "租户管理",
                path: "/tenant",
                component: "@/pages/tenant/List",
                icon: Some("ApartmentOutlined"),
                parent_path: Some("/settings"),
                sort_order: 1,
                status: "active",
                roles: vec!["admin"],
                i18n_key: Some("menu.tenant"),
            },
            DefaultMenu {
                name: "用户管理",
                path: "/users",
                component: "@/pages/user/List",
                icon: Some("UserOutlined"),
                parent_path: Some("/settings"),
                sort_order: 2,
                status: "active",
                roles: vec!["admin"],
                i18n_key: Some("menu.user"),
            },
            DefaultMenu {
                name: "组织机构",
                path: "/organization",
                component: "@/pages/organization/List",
                icon: Some("ApartmentOutlined"),
                parent_path: Some("/settings"),
                sort_order: 3,
                status: "active",
                roles: vec!["admin", "editor"],
                i18n_key: Some("menu.organization"),
            },
            DefaultMenu {
                name: "部门管理",
                path: "/department",
                component: "@/pages/department/List",
                icon: Some("TeamOutlined"),
                parent_path: Some("/settings"),
                sort_order: 4,
                status: "active",
                roles: vec!["admin", "editor"],
                i18n_key: Some("menu.department"),
            },
            DefaultMenu {
                name: "站点管理",
                path: "/site",
                component: "@/pages/site/List",
                icon: Some("EnvironmentOutlined"),
                parent_path: Some("/settings"),
                sort_order: 5,
                status: "active",
                roles: vec!["admin", "editor"],
                i18n_key: Some("menu.site"),
            },
            DefaultMenu {
                name: "命名空间",
                path: "/namespace",
                component: "@/pages/namespace/List",
                icon: Some("FolderOutlined"),
                parent_path: Some("/settings"),
                sort_order: 6,
                status: "active",
                roles: vec!["admin", "editor"],
                i18n_key: Some("menu.namespace"),
            },
            DefaultMenu {
                name: "角色管理",
                path: "/settings/role",
                component: "@/pages/role/List",
                icon: Some("TeamOutlined"),
                parent_path: Some("/settings"),
                sort_order: 7,
                status: "active",
                roles: vec!["admin"],
                i18n_key: Some("menu.role"),
            },
            DefaultMenu {
                name: "用户角色授权",
                path: "/settings/user-role",
                component: "@/pages/user-role/List",
                icon: Some("UserOutlined"),
                parent_path: Some("/settings"),
                sort_order: 8,
                status: "active",
                roles: vec!["admin"],
                i18n_key: Some("menu.userRole"),
            },
            DefaultMenu {
                name: "角色菜单授权",
                path: "/settings/role-menu",
                component: "@/pages/role-menu/List",
                icon: Some("MenuOutlined"),
                parent_path: Some("/settings"),
                sort_order: 9,
                status: "active",
                roles: vec!["admin"],
                i18n_key: Some("menu.roleMenu"),
            },
            // 资源管理二级菜单
            DefaultMenu {
                name: "CRD定义",
                path: "/resources/crd",
                component: "@/pages/resource/crd/List",
                icon: Some("CodeOutlined"),
                parent_path: Some("/resources"),
                sort_order: 1,
                status: "active",
                roles: vec!["admin", "editor"],
                i18n_key: Some("menu.resource.crd"),
            },
            DefaultMenu {
                name: "Operator定义",
                path: "/resources/operator",
                component: "@/pages/resource/operator/List",
                icon: Some("ControlOutlined"),
                parent_path: Some("/resources"),
                sort_order: 2,
                status: "active",
                roles: vec!["admin", "editor"],
                i18n_key: Some("menu.resource.operator"),
            },
            DefaultMenu {
                name: "Controller定义",
                path: "/resources/controller",
                component: "@/pages/resource/controller/List",
                icon: Some("GatewayOutlined"),
                parent_path: Some("/resources"),
                sort_order: 3,
                status: "active",
                roles: vec!["admin", "editor"],
                i18n_key: Some("menu.resource.controller"),
            },
            // 配置管理二级菜单
            DefaultMenu {
                name: "配置项",
                path: "/config/configmap",
                component: "@/pages/config/config_map/List",
                icon: Some("FileTextOutlined"),
                parent_path: Some("/config"),
                sort_order: 1,
                status: "active",
                roles: vec!["admin", "editor"],
                i18n_key: Some("menu.config.configMap"),
            },
            DefaultMenu {
                name: "保密字典",
                path: "/config/secret",
                component: "@/pages/config/secret/List",
                icon: Some("LockOutlined"),
                parent_path: Some("/config"),
                sort_order: 2,
                status: "active",
                roles: vec!["admin", "editor"],
                i18n_key: Some("menu.config.secret"),
            },
            // 设备管理二级菜单
            DefaultMenu {
                name: "产品管理",
                path: "/device/product",
                component: "@/pages/device/product/List",
                icon: Some("BoxOutlined"),
                parent_path: Some("/device"),
                sort_order: 1,
                status: "active",
                roles: vec!["admin", "editor"],
                i18n_key: Some("menu.device.product"),
            },
            DefaultMenu {
                name: "驱动管理",
                path: "/device/driver",
                component: "@/pages/device/driver/List",
                icon: Some("HddOutlined"),
                parent_path: Some("/device"),
                sort_order: 2,
                status: "active",
                roles: vec!["admin", "editor"],
                i18n_key: Some("menu.device.driver"),
            },
            DefaultMenu {
                name: "节点管理",
                path: "/device/node",
                component: "@/pages/device/node/List",
                icon: Some("ClusterOutlined"),
                parent_path: Some("/device"),
                sort_order: 3,
                status: "active",
                roles: vec!["admin", "editor"],
                i18n_key: Some("menu.device.node"),
            },
            DefaultMenu {
                name: "设备实例",
                path: "/device/instance",
                component: "@/pages/device/instance/List",
                icon: Some("DesktopOutlined"),
                parent_path: Some("/device"),
                sort_order: 5,
                status: "inactive",
                roles: vec!["admin", "editor"],
                i18n_key: Some("menu.device.instance"),
            },
            DefaultMenu {
                name: "设备定义",
                path: "/device/device",
                component: "@/pages/device/device/List",
                icon: Some("DesktopOutlined"),
                parent_path: Some("/device"),
                sort_order: 4,
                status: "active",
                roles: vec!["admin", "editor"],
                i18n_key: Some("menu.device.device"),
            },
        ];

        // 获取所有已存在的菜单路径
         let existing_menus = menu::Entity::find()
             .all(db)
             .await?;
         let existing_paths: std::collections::HashSet<String> = existing_menus
             .iter()
             .map(|m| m.path.clone())
             .collect();
 
         // 存储已创建的菜单路径到ID映射
         let mut path_to_id: std::collections::HashMap<String, Uuid> = existing_menus
             .iter()
             .map(|m| (m.path.clone(), m.id))
             .collect();

        // 按层级顺序处理菜单（先一级，后二级）
             for menu_def in default_menus {
                 if existing_paths.contains(menu_def.path) {
                     // 更新现有菜单的状态
                     if let Some(existing_menu) = existing_menus.iter().find(|m| m.path == menu_def.path) {
                         if existing_menu.status != menu_def.status {
                             tracing::info!("Updating menu status: {} - {} -> {}", menu_def.path, existing_menu.status, menu_def.status);
                             let mut active_model: menu::ActiveModel = existing_menu.clone().into();
                             active_model.status = Set(menu_def.status.to_string());
                             active_model.update(db).await?;
                         }
                     }
                     continue;
                 }

                 tracing::info!("Adding missing menu: {} - {}", menu_def.path, menu_def.name);

                 let parent_id = if let Some(parent_path) = menu_def.parent_path {
                     path_to_id.get(parent_path).cloned()
                 } else {
                     None
                 };

                 let menu_id = Uuid::new_v4();
                 let mut active_model = menu::ActiveModel {
                     id: Set(menu_id),
                     parent_id: Set(parent_id),
                     name: Set(menu_def.name.to_string()),
                     path: Set(menu_def.path.to_string()),
                     component: Set(menu_def.component.to_string()),
                     icon: Set(menu_def.icon.map(|s| s.to_string())),
                     sort_order: Set(menu_def.sort_order),
                     status: Set(menu_def.status.to_string()),
                     roles: Set(serde_json::json!(menu_def.roles)),
                     i18n_key: Set(menu_def.i18n_key.map(|s| s.to_string())),
                     created_at: Set(now),
                     updated_at: Set(now),
                 };
                 active_model.insert(db).await?;
                 path_to_id.insert(menu_def.path.to_string(), menu_id);
             }

        // 检查是否已有角色
        let role_count = role::Entity::find().count(db).await?;
        if role_count == 0 {
            // 获取默认租户ID
            let default_tenant = crate::entity::tenant::Entity::find()
                .filter(crate::entity::tenant::Column::Slug.eq("default"))
                .one(db)
                .await?
                .ok_or_else(|| DbErr::Custom("Default tenant not found".to_string()))?;

            // 初始化角色
            let admin_role_id = Uuid::new_v4();
            let admin_role = role::ActiveModel {
                id: Set(admin_role_id),
                tenant_id: Set(default_tenant.id),
                name: Set("系统管理员".to_string()),
                slug: Set("admin".to_string()),
                description: Set(Some("系统管理员，拥有所有权限".to_string())),
                permissions: Set(Json::Array(vec![])),
                status: Set("active".to_string()),
                created_at: Set(now),
                updated_at: Set(now),
            };
            admin_role.insert(db).await?;

            let editor_role_id = Uuid::new_v4();
            let editor_role = role::ActiveModel {
                id: Set(editor_role_id),
                tenant_id: Set(default_tenant.id),
                name: Set("编辑".to_string()),
                slug: Set("editor".to_string()),
                description: Set(Some("编辑，可以编辑内容".to_string())),
                permissions: Set(Json::Array(vec![])),
                status: Set("active".to_string()),
                created_at: Set(now),
                updated_at: Set(now),
            };
            editor_role.insert(db).await?;

            let user_role_id = Uuid::new_v4();
            let user_role = role::ActiveModel {
                id: Set(user_role_id),
                tenant_id: Set(default_tenant.id),
                name: Set("普通用户".to_string()),
                slug: Set("user".to_string()),
                description: Set(Some("普通用户，只有基本权限".to_string())),
                permissions: Set(Json::Array(vec![])),
                status: Set("active".to_string()),
                created_at: Set(now),
                updated_at: Set(now),
            };
            user_role.insert(db).await?;

            // 为admin角色分配所有菜单权限
            let all_menus = menu::Entity::find().all(db).await?;
            for menu in all_menus {
                let role_menu = role_menu::ActiveModel {
                    role_id: Set(admin_role_id),
                    menu_id: Set(menu.id),
                    ..Default::default()
                };
                role_menu.insert(db).await?;
            }
        } else {
            // 如果角色已存在，确保admin角色拥有所有菜单权限
            // 查找admin角色
            if let Some(admin_role) = role::Entity::find()
                .filter(role::Column::Slug.eq("admin"))
                .one(db)
                .await?
            {
                // 获取所有菜单
                let all_menus = menu::Entity::find().all(db).await?;
                // 获取admin角色已有的菜单权限
                let existing_role_menus: Vec<Uuid> = role_menu::Entity::find()
                    .filter(role_menu::Column::RoleId.eq(admin_role.id))
                    .all(db)
                    .await?
                    .into_iter()
                    .map(|rm| rm.menu_id)
                    .collect();

                // 为admin角色添加缺失的菜单权限
                for menu in all_menus {
                    if !existing_role_menus.contains(&menu.id) {
                        tracing::info!("Adding missing menu permission for admin: {} ({})", menu.path, menu.name);
                        let role_menu = role_menu::ActiveModel {
                            role_id: Set(admin_role.id),
                            menu_id: Set(menu.id),
                            ..Default::default()
                        };
                        role_menu.insert(db).await?;
                    }
                }
            }
        }

        Ok(())
    }
}