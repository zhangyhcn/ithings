# MES制造执行系统设计文档

## 1. 系统概述

MES（Manufacturing Execution System）制造执行系统是位于上层计划管理系统（ERP）与底层工业控制系统之间的车间级管理信息系统。它为操作人员和管理人员提供计划的执行、跟踪以及所有资源（人员、设备、物料、工艺等）的当前状态信息。

### 1.1 系统定位

```
┌─────────────────────────────────────────────────────────┐
│                      ERP系统                             │
│              (企业资源计划/计划层)                         │
└─────────────────────────┬───────────────────────────────┘
                          │ 生产计划/订单
                          ▼
┌─────────────────────────────────────────────────────────┐
│                      MES系统                             │
│              (制造执行系统/执行层)                         │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐       │
│  │工单管理 │ │生产调度 │ │质量管理 │ │设备管理 │       │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘       │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐       │
│  │物料管理 │ │工艺路线 │ │人员管理 │ │报表看板 │       │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘       │
└─────────────────────────┬───────────────────────────────┘
                          │ 生产指令/参数
                          ▼
┌─────────────────────────────────────────────────────────┐
│                    SCADA/PLC/DCS                         │
│              (工业控制系统/控制层)                         │
└─────────────────────────────────────────────────────────┘
```

### 1.2 核心价值

- **实时监控**：实时采集生产现场数据，监控生产进度
- **质量追溯**：完整记录生产过程，实现产品质量追溯
- **资源优化**：优化人员、设备、物料等资源配置
- **决策支持**：提供数据分析和可视化，支持管理决策

## 2. 功能模块设计

### 2.1 模块总览

| 模块名称 | 功能描述 | 核心实体 |
|---------|---------|---------|
| 产品管理 | 产品定义、物模型、规则配置 | 产品 |
| 工单管理 | 生产订单接收、分解、执行跟踪 | 工单、工单明细 |
| 生产调度 | 生产任务排程、资源分配 | 排程计划、任务分配 |
| 工艺路线 | 产品工艺定义、工序管理 | 工艺路线、工序 |
| 物料管理 | 物料库存、领料、退料 | 物料、库存、出入库单 |
| 制造过程 | 生产执行、数据采集 | 生产记录、工站 |
| 质量管理 | 质量检验、不良处理 | 检验单、不良记录 |
| 设备管理 | 设备台账、维护保养 | 设备、保养计划 |
| 人员管理 | 人员信息、考勤、技能 | 员工、技能证书 |
| 报表看板 | 数据统计、可视化展示 | 报表配置、看板 |

### 2.2 模块详细设计

#### 2.2.1 产品管理模块

**功能描述**：
- 产品基础信息管理
- 产品规格型号定义
- 产品BOM管理
- 产品与工艺路线关联

**核心实体**：
```
产品 (Product)
├── id: UUID                    # 产品ID
├── tenant_id: UUID             # 租户ID
├── product_no: String          # 产品编号
├── product_name: String        # 产品名称
├── specification: String       # 规格型号
├── unit: String                # 单位
├── product_type: Enum          # 产品类型(成品/半成品/原料)
├── description: String         # 产品描述
├── status: Enum                # 状态(启用/停用)
├── created_at: DateTime        # 创建时间
└── updated_at: DateTime        # 更新时间
```

**说明**：
- MES中的产品是指要生产制造的实体产品
- 与物联网平台的产品概念不同，不包含物模型和规则配置
- 产品编号用于唯一标识产品，便于追溯和管理
- 产品类型区分成品、半成品和原料，用于不同业务场景

#### 2.2.2 工单管理模块

**功能描述**：
- 接收ERP下达的生产订单
- 工单创建、修改、关闭
- 工单进度跟踪
- 工单完工汇报

**核心实体**：
```
工单 (WorkOrder)
├── id: UUID                    # 工单ID
├── order_no: String            # 工单编号
├── erp_order_no: String        # ERP订单号
├── product_id: UUID            # 产品ID
├── product_name: String        # 产品名称
├── quantity: Decimal           # 计划数量
├── completed_qty: Decimal      # 完成数量
├── status: Enum                # 状态(待排程/生产中/已完成/已关闭)
├── plan_start_time: DateTime   # 计划开始时间
├── plan_end_time: DateTime     # 计划结束时间
├── actual_start_time: DateTime # 实际开始时间
├── actual_end_time: DateTime   # 实际结束时间
├── priority: Integer           # 优先级
├── workshop_id: UUID           # 车间ID
├── production_line_id: UUID    # 产线ID
└── created_at: DateTime        # 创建时间
```

#### 2.2.2 生产调度模块

**功能描述**：
- 生产任务排程
- 设备/人员资源分配
- 排程调整优化
- 任务下达

**核心实体**：
```
排程计划 (SchedulePlan)
├── id: UUID                    # 排程ID
├── plan_no: String             # 排程编号
├── work_order_id: UUID         # 工单ID
├── process_id: UUID            # 工序ID
├── equipment_id: UUID          # 设备ID
├── operator_id: UUID           # 操作员ID
├── plan_quantity: Decimal      # 计划数量
├── status: Enum                # 状态
├── start_time: DateTime        # 开始时间
├── end_time: DateTime          # 结束时间
└── created_at: DateTime        # 创建时间
```

#### 2.2.3 工艺路线模块

**功能描述**：
- 产品工艺路线定义
- 工序参数设置
- 工序依赖关系
- 工艺版本管理

**核心实体**：
```
工艺路线 (ProcessRoute)
├── id: UUID                    # 工艺路线ID
├── product_id: UUID            # 产品ID
├── route_name: String          # 路线名称
├── version: String             # 版本号
├── status: Enum                # 状态(草稿/生效/失效)
├── is_default: Boolean         # 是否默认
└── created_at: DateTime        # 创建时间

工序 (Process)
├── id: UUID                    # 工序ID
├── route_id: UUID              # 工艺路线ID
├── process_no: String          # 工序编号
├── process_name: String        # 工序名称
├── sequence: Integer           # 顺序
├── work_station_id: UUID       # 工站ID
├── standard_time: Decimal      # 标准工时(分钟)
├── setup_time: Decimal         # 准备时间(分钟)
├── process_params: JSON        # 工艺参数
└── next_process_id: UUID       # 下一工序ID
```

#### 2.2.4 物料管理模块

**功能描述**：
- 物料基础信息管理
- 库存管理
- 领料/退料管理
- 物料追溯

**核心实体**：
```
物料 (Material)
├── id: UUID                    # 物料ID
├── material_no: String         # 物料编号
├── material_name: String       # 物料名称
├── specification: String       # 规格
├── unit: String                # 单位
├── material_type: Enum         # 类型(原料/半成品/成品)
├── safety_stock: Decimal       # 安全库存
├── max_stock: Decimal          # 最大库存
└── created_at: DateTime        # 创建时间

库存 (Inventory)
├── id: UUID                    # 库存ID
├── material_id: UUID           # 物料ID
├── warehouse_id: UUID          # 仓库ID
├── location_id: UUID           # 库位ID
├── batch_no: String            # 批次号
├── quantity: Decimal           # 数量
├── locked_qty: Decimal         # 锁定数量
└── updated_at: DateTime        # 更新时间

出入库单 (StockMovement)
├── id: UUID                    # 单据ID
├── movement_no: String         # 单据编号
├── movement_type: Enum         # 类型(领料/退料/入库/出库)
├── work_order_id: UUID         # 关联工单
├── material_id: UUID           # 物料ID
├── quantity: Decimal           # 数量
├── batch_no: String            # 批次号
├── operator_id: UUID           # 操作员
├── status: Enum                # 状态
└── created_at: DateTime        # 创建时间
```

#### 2.2.5 制造过程模块

**功能描述**：
- 生产执行记录
- 数据采集
- 过程追溯
- 异常处理

**核心实体**：
```
生产记录 (ProductionRecord)
├── id: UUID                    # 记录ID
├── work_order_id: UUID         # 工单ID
├── process_id: UUID            # 工序ID
├── equipment_id: UUID          # 设备ID
├── operator_id: UUID           # 操作员ID
├── batch_no: String            # 批次号
├── quantity: Decimal           # 生产数量
├── good_qty: Decimal           # 良品数
├── defect_qty: Decimal         # 不良数
├── start_time: DateTime        # 开始时间
├── end_time: DateTime          # 结束时间
├── process_data: JSON          # 过程数据
└── created_at: DateTime        # 创建时间

工站 (WorkStation)
├── id: UUID                    # 工站ID
├── station_no: String          # 工站编号
├── station_name: String        # 工站名称
├── workshop_id: UUID           # 车间ID
├── production_line_id: UUID    # 产线ID
├── equipment_id: UUID          # 关联设备
├── status: Enum                # 状态
└── created_at: DateTime        # 创建时间
```

#### 2.2.6 质量管理模块

**功能描述**：
- 来料检验(IQC)
- 过程检验(IPQC)
- 成品检验(FQC)
- 不良品处理
- 质量追溯

**核心实体**：
```
检验单 (InspectionOrder)
├── id: UUID                    # 检验单ID
├── inspection_no: String       # 检验单号
├── inspection_type: Enum       # 类型(IQC/IPQC/FQC)
├── work_order_id: UUID         # 关联工单
├── material_id: UUID           # 物料ID
├── batch_no: String            # 批次号
├── sample_qty: Integer         # 抽样数量
├── pass_qty: Integer           # 合格数量
├── defect_qty: Integer         # 不良数量
├── result: Enum                # 结果(合格/不合格/待定)
├── inspector_id: UUID          # 检验员
├── inspect_time: DateTime      # 检验时间
└── created_at: DateTime        # 创建时间

不良记录 (DefectRecord)
├── id: UUID                    # 记录ID
├── inspection_id: UUID         # 检验单ID
├── defect_type_id: UUID        # 不良类型ID
├── defect_code: String         # 不良代码
├── quantity: Integer           # 数量
├── description: String         # 描述
├── disposition: Enum           # 处置方式(返工/报废/特采)
├── status: Enum                # 状态
└── created_at: DateTime        # 创建时间
```

#### 2.2.7 设备管理模块

**功能描述**：
- 设备台账管理
- 设备状态监控
- 维护保养计划
- 故障维修管理

**核心实体**：
```
设备 (Equipment)
├── id: UUID                    # 设备ID
├── equipment_no: String        # 设备编号
├── equipment_name: String      # 设备名称
├── equipment_type: String      # 设备类型
├── model: String               # 型号
├── manufacturer: String        # 制造商
├── purchase_date: Date         # 购置日期
├── workshop_id: UUID           # 所属车间
├── status: Enum                # 状态(运行/待机/维修/报废)
├── ip_address: String          # IP地址
└── created_at: DateTime        # 创建时间

保养计划 (MaintenancePlan)
├── id: UUID                    # 计划ID
├── equipment_id: UUID          # 设备ID
├── plan_type: Enum             # 类型(日常/周保/月保/年保)
├── plan_date: Date             # 计划日期
├── content: String             # 保养内容
├── status: Enum                # 状态
├── executor_id: UUID           # 执行人
├── execute_time: DateTime      # 执行时间
└── created_at: DateTime        # 创建时间
```

#### 2.2.8 人员管理模块

**功能描述**：
- 员工信息管理
- 技能资质管理
- 考勤管理
- 绩效统计

**核心实体**：
```
员工 (Employee)
├── id: UUID                    # 员工ID
├── employee_no: String         # 工号
├── name: String                # 姓名
├── department_id: UUID         # 部门ID
├── position: String            # 职位
├── phone: String               # 电话
├── status: Enum                # 状态(在职/离职)
├── entry_date: Date            # 入职日期
└── created_at: DateTime        # 创建时间

技能证书 (SkillCertificate)
├── id: UUID                    # 证书ID
├── employee_id: UUID           # 员工ID
├── skill_type: String          # 技能类型
├── certificate_no: String      # 证书编号
├── level: String               # 等级
├── issue_date: Date            # 发证日期
├── expire_date: Date           # 有效期
└── created_at: DateTime        # 创建时间
```

#### 2.2.9 报表看板模块

**功能描述**：
- 生产进度看板
- 设备状态看板
- 质量分析报表
- 效率统计报表

**核心实体**：
```
看板配置 (DashboardConfig)
├── id: UUID                    # 配置ID
├── name: String                # 看板名称
├── type: Enum                  # 类型
├── config: JSON                # 配置内容
├── refresh_interval: Integer   # 刷新间隔(秒)
└── created_at: DateTime        # 创建时间
```

## 3. ER图

### 3.1 核心实体关系图

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              MES系统ER图                                      │
└─────────────────────────────────────────────────────────────────────────────┘

                              ┌──────────────┐
                              │   产品(Product)   │
                              │──────────────│
                              │ id           │
                              │ product_no   │
                              │ name         │
                              │ specification│
                              └──────┬───────┘
                                     │
         ┌───────────────────────────┼───────────────────────────┐
         │                           │                           │
         ▼                           ▼                           ▼
┌────────────────┐          ┌────────────────┐          ┌────────────────┐
│  工单(WorkOrder) │          │ 工艺路线(ProcessRoute)│          │ 物料清单(BOM)   │
│────────────────│          │────────────────│          │────────────────│
│ id             │          │ id             │          │ id             │
│ order_no       │          │ product_id     │          │ product_id     │
│ product_id(FK) │          │ route_name     │          │ material_id    │
│ quantity       │          │ version        │          │ quantity       │
│ status         │          │ status         │          │────────────────│
└───────┬────────┘          └───────┬────────┘          └────────────────┘
        │                           │
        │                           │
        ▼                           ▼
┌────────────────┐          ┌────────────────┐
│ 排程计划(Schedule)│◄────────│   工序(Process)   │
│────────────────│          │────────────────│
│ id             │          │ id             │
│ work_order_id  │          │ route_id(FK)   │
│ process_id(FK) │          │ process_name   │
│ equipment_id   │          │ sequence       │
│ operator_id    │          │ standard_time  │
│ start_time     │          │ work_station_id│
└───────┬────────┘          └───────┬────────┘
        │                           │
        │                           │
        ▼                           ▼
┌────────────────┐          ┌────────────────┐
│生产记录(Production)│        │  工站(WorkStation) │
│────────────────│          │────────────────│
│ id             │          │ id             │
│ work_order_id  │          │ station_no     │
│ process_id     │          │ station_name   │
│ equipment_id   │          │ workshop_id    │
│ operator_id    │          │ equipment_id   │
│ quantity       │          └────────────────┘
│ good_qty       │
│ defect_qty     │
└───────┬────────┘
        │
        ├───────────────────────────┐
        │                           │
        ▼                           ▼
┌────────────────┐          ┌────────────────┐
│检验单(Inspection)│          │不良记录(Defect)   │
│────────────────│          │────────────────│
│ id             │          │ id             │
│ inspection_no  │          │ inspection_id  │
│ inspection_type│          │ defect_type_id │
│ work_order_id  │          │ quantity       │
│ batch_no       │          │ disposition    │
│ result         │          └────────────────┘
└────────────────┘

┌────────────────┐          ┌────────────────┐          ┌────────────────┐
│   物料(Material)  │◄────────│   库存(Inventory)  │──────────│   仓库(Warehouse)  │
│────────────────│          │────────────────│          │────────────────│
│ id             │          │ id             │          │ id             │
│ material_no    │          │ material_id(FK)│          │ warehouse_no   │
│ material_name  │          │ warehouse_id   │          │ warehouse_name │
│ specification  │          │ location_id    │          │ type           │
│ unit           │          │ batch_no       │          └────────────────┘
│ material_type  │          │ quantity       │
└───────┬────────┘          └────────────────┘
        │
        ▼
┌────────────────┐
│出入库单(StockMovement)│
│────────────────│
│ id             │
│ movement_type  │
│ material_id(FK)│
│ quantity       │
│ batch_no       │
│ operator_id    │
└────────────────┘

┌────────────────┐          ┌────────────────┐          ┌────────────────┐
│   设备(Equipment)  │          │ 保养计划(Maintenance)│          │   员工(Employee)   │
│────────────────│          │────────────────│          │────────────────│
│ id             │◄─────────│ id             │          │ id             │
│ equipment_no   │          │ equipment_id   │          │ employee_no    │
│ equipment_name │          │ plan_type      │          │ name           │
│ equipment_type │          │ plan_date      │          │ department_id  │
│ model          │          │ content        │          │ position       │
│ manufacturer   │          │ executor_id    ├──────────│ status         │
│ status         │          └────────────────┘          └───────┬────────┘
└────────────────┘                                              │
                                                                ▼
                                                      ┌────────────────┐
                                                      │技能证书(Skill)   │
                                                      │────────────────│
                                                      │ id             │
                                                      │ employee_id    │
                                                      │ skill_type     │
                                                      │ certificate_no │
                                                      │ expire_date    │
                                                      └────────────────┘
```

### 3.2 模块关系图

```
                    ┌─────────────┐
                    │   ERP系统    │
                    └──────┬──────┘
                           │ 生产订单
                           ▼
┌──────────────────────────────────────────────────────────────┐
│                        工单管理                                │
│  ┌─────────┐    ┌─────────┐    ┌─────────┐                  │
│  │订单接收  │───▶│工单创建  │───▶│进度跟踪  │                  │
│  └─────────┘    └─────────┘    └─────────┘                  │
└──────────────────────────┬───────────────────────────────────┘
                           │
         ┌─────────────────┼─────────────────┐
         │                 │                 │
         ▼                 ▼                 ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  生产调度    │    │  工艺路线    │    │  物料管理    │
│─────────────│    │─────────────│    │─────────────│
│ 任务排程    │    │ 工序定义    │    │ 领料管理    │
│ 资源分配    │    │ 参数设置    │    │ 库存管理    │
│ 任务下达    │    │ 版本管理    │    │ 物料追溯    │
└──────┬──────┘    └──────┬──────┘    └──────┬──────┘
       │                  │                  │
       └─────────────────┬┴──────────────────┘
                         │
                         ▼
              ┌─────────────────────┐
              │    制造过程管理      │
              │─────────────────────│
              │ 生产执行            │
              │ 数据采集            │
              │ 过程追溯            │
              └──────────┬──────────┘
                         │
         ┌───────────────┼───────────────┐
         │               │               │
         ▼               ▼               ▼
┌─────────────┐   ┌─────────────┐   ┌─────────────┐
│   质量管理   │   │   设备管理   │   │   人员管理   │
│─────────────│   │─────────────│   │─────────────│
│ 来料检验    │   │ 设备台账    │   │ 员工信息    │
│ 过程检验    │   │ 状态监控    │   │ 技能资质    │
│ 成品检验    │   │ 维护保养    │   │ 考勤管理    │
│ 不良处理    │   │ 故障维修    │   │ 绩效统计    │
└──────┬──────┘   └──────┬──────┘   └──────┬──────┘
       │                 │                 │
       └─────────────────┼─────────────────┘
                         │
                         ▼
              ┌─────────────────────┐
              │     报表与看板       │
              │─────────────────────│
              │ 生产进度看板         │
              │ 设备状态看板         │
              │ 质量分析报表         │
              │ 效率统计报表         │
              └─────────────────────┘
```

## 4. 数据库设计

### 4.1 表清单

| 序号 | 表名 | 中文名 | 所属模块 |
|-----|------|-------|---------|
| 1 | products | 产品表 | 基础数据 |
| 2 | work_orders | 工单表 | 工单管理 |
| 3 | process_routes | 工艺路线表 | 工艺路线 |
| 4 | processes | 工序表 | 工艺路线 |
| 5 | schedule_plans | 排程计划表 | 生产调度 |
| 6 | materials | 物料表 | 物料管理 |
| 7 | warehouses | 仓库表 | 物料管理 |
| 8 | locations | 库位表 | 物料管理 |
| 9 | inventories | 库存表 | 物料管理 |
| 10 | stock_movements | 出入库单表 | 物料管理 |
| 11 | work_stations | 工站表 | 制造过程 |
| 12 | production_records | 生产记录表 | 制造过程 |
| 13 | inspection_orders | 检验单表 | 质量管理 |
| 14 | defect_types | 不良类型表 | 质量管理 |
| 15 | defect_records | 不良记录表 | 质量管理 |
| 16 | equipments | 设备表 | 设备管理 |
| 17 | maintenance_plans | 保养计划表 | 设备管理 |
| 18 | employees | 员工表 | 人员管理 |
| 19 | departments | 部门表 | 人员管理 |
| 20 | skill_certificates | 技能证书表 | 人员管理 |
| 21 | workshops | 车间表 | 基础数据 |
| 22 | production_lines | 产线表 | 基础数据 |

### 4.2 核心表DDL

```sql
-- 产品表
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_no VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(200) NOT NULL,
    specification VARCHAR(200),
    unit VARCHAR(20),
    product_type VARCHAR(50),
    status VARCHAR(20) DEFAULT 'active',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 工单表
CREATE TABLE work_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_no VARCHAR(50) NOT NULL UNIQUE,
    erp_order_no VARCHAR(50),
    product_id UUID REFERENCES products(id),
    product_name VARCHAR(200),
    quantity DECIMAL(18,4) NOT NULL,
    completed_qty DECIMAL(18,4) DEFAULT 0,
    status VARCHAR(20) DEFAULT 'pending',
    priority INTEGER DEFAULT 0,
    plan_start_time TIMESTAMP,
    plan_end_time TIMESTAMP,
    actual_start_time TIMESTAMP,
    actual_end_time TIMESTAMP,
    workshop_id UUID,
    production_line_id UUID,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 工艺路线表
CREATE TABLE process_routes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID REFERENCES products(id),
    route_name VARCHAR(100) NOT NULL,
    version VARCHAR(20) DEFAULT '1.0',
    status VARCHAR(20) DEFAULT 'draft',
    is_default BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 工序表
CREATE TABLE processes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    route_id UUID REFERENCES process_routes(id),
    process_no VARCHAR(20) NOT NULL,
    process_name VARCHAR(100) NOT NULL,
    sequence INTEGER NOT NULL,
    work_station_id UUID,
    standard_time DECIMAL(10,2),
    setup_time DECIMAL(10,2),
    process_params JSONB,
    next_process_id UUID REFERENCES processes(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 物料表
CREATE TABLE materials (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    material_no VARCHAR(50) NOT NULL UNIQUE,
    material_name VARCHAR(200) NOT NULL,
    specification VARCHAR(200),
    unit VARCHAR(20),
    material_type VARCHAR(50),
    safety_stock DECIMAL(18,4),
    max_stock DECIMAL(18,4),
    status VARCHAR(20) DEFAULT 'active',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 库存表
CREATE TABLE inventories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    material_id UUID REFERENCES materials(id),
    warehouse_id UUID,
    location_id UUID,
    batch_no VARCHAR(50),
    quantity DECIMAL(18,4) DEFAULT 0,
    locked_qty DECIMAL(18,4) DEFAULT 0,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(material_id, warehouse_id, location_id, batch_no)
);

-- 生产记录表
CREATE TABLE production_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    work_order_id UUID REFERENCES work_orders(id),
    process_id UUID REFERENCES processes(id),
    equipment_id UUID,
    operator_id UUID,
    batch_no VARCHAR(50),
    quantity DECIMAL(18,4) NOT NULL,
    good_qty DECIMAL(18,4),
    defect_qty DECIMAL(18,4),
    start_time TIMESTAMP,
    end_time TIMESTAMP,
    process_data JSONB,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 设备表
CREATE TABLE equipments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    equipment_no VARCHAR(50) NOT NULL UNIQUE,
    equipment_name VARCHAR(100) NOT NULL,
    equipment_type VARCHAR(50),
    model VARCHAR(100),
    manufacturer VARCHAR(100),
    purchase_date DATE,
    workshop_id UUID,
    status VARCHAR(20) DEFAULT 'idle',
    ip_address VARCHAR(50),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 员工表
CREATE TABLE employees (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_no VARCHAR(20) NOT NULL UNIQUE,
    name VARCHAR(50) NOT NULL,
    department_id UUID,
    position VARCHAR(50),
    phone VARCHAR(20),
    status VARCHAR(20) DEFAULT 'active',
    entry_date DATE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## 5. 接口设计

### 5.1 API概览

| 模块 | 接口路径 | 方法 | 描述 |
|-----|---------|-----|------|
| 工单管理 | /api/v1/work-orders | GET | 获取工单列表 |
| 工单管理 | /api/v1/work-orders | POST | 创建工单 |
| 工单管理 | /api/v1/work-orders/{id} | GET | 获取工单详情 |
| 工单管理 | /api/v1/work-orders/{id} | PUT | 更新工单 |
| 工单管理 | /api/v1/work-orders/{id}/start | POST | 开始生产 |
| 工单管理 | /api/v1/work-orders/{id}/complete | POST | 完工汇报 |
| 生产调度 | /api/v1/schedules | GET | 获取排程列表 |
| 生产调度 | /api/v1/schedules | POST | 创建排程 |
| 物料管理 | /api/v1/materials | GET | 获取物料列表 |
| 物料管理 | /api/v1/inventories | GET | 获取库存列表 |
| 物料管理 | /api/v1/stock-movements | POST | 出入库操作 |
| 质量管理 | /api/v1/inspections | GET | 获取检验单列表 |
| 质量管理 | /api/v1/inspections | POST | 创建检验单 |
| 设备管理 | /api/v1/equipments | GET | 获取设备列表 |
| 设备管理 | /api/v1/equipments/{id}/status | GET | 获取设备状态 |
| 人员管理 | /api/v1/employees | GET | 获取员工列表 |

### 5.2 核心接口示例

#### 创建工单

```json
POST /api/v1/work-orders
Content-Type: application/json

{
    "erp_order_no": "ERP2024010001",
    "product_id": "uuid",
    "quantity": 1000,
    "priority": 1,
    "plan_start_time": "2024-01-15T08:00:00Z",
    "plan_end_time": "2024-01-15T17:00:00Z",
    "workshop_id": "uuid",
    "production_line_id": "uuid"
}

Response:
{
    "code": 200,
    "message": "success",
    "data": {
        "id": "uuid",
        "order_no": "WO2024010001",
        "status": "pending",
        ...
    }
}
```

#### 完工汇报

```json
POST /api/v1/work-orders/{id}/complete
Content-Type: application/json

{
    "process_id": "uuid",
    "equipment_id": "uuid",
    "operator_id": "uuid",
    "quantity": 100,
    "good_qty": 98,
    "defect_qty": 2,
    "process_data": {
        "temperature": 25.5,
        "pressure": 1.2
    }
}
```

## 6. 技术架构

### 6.1 技术栈

| 层次 | 技术选型 | 说明 |
|-----|---------|------|
| 前端 | React + Ant Design Pro | 企业级前端框架 |
| 后端 | Rust + Axum | 高性能Web框架 |
| 数据库 | PostgreSQL | 关系型数据库 |
| 缓存 | Redis | 缓存/消息队列 |
| 消息队列 | Kafka | 事件驱动 |
| 时序数据库 | TimescaleDB | 设备数据存储 |
| 容器化 | Docker + Kubernetes | 部署运维 |

### 6.2 系统架构图

```
┌─────────────────────────────────────────────────────────────────┐
│                         客户端层                                  │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐            │
│  │ Web端   │  │ 移动端   │  │ 大屏端   │  │ PDA设备  │            │
│  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘            │
└───────┼────────────┼────────────┼────────────┼──────────────────┘
        │            │            │            │
        └────────────┴─────┬──────┴────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│                        网关层 (Nginx/Kong)                       │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                         服务层                                    │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                    MES API Service                        │   │
│  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ │   │
│  │  │工单服务│ │调度服务│ │物料服务│ │质量服务│ │设备服务│ │   │
│  │  └────────┘ └────────┘ └────────┘ └────────┘ └────────┘ │   │
│  └──────────────────────────────────────────────────────────┘   │
└───────────────────────────┬─────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
        ▼                   ▼                   ▼
┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│ PostgreSQL   │    │    Redis     │    │    Kafka     │
│   关系数据库  │    │   缓存/队列   │    │   消息队列   │
└──────────────┘    └──────────────┘    └──────────────┘
        │
        ▼
┌──────────────┐
│ TimescaleDB  │
│  时序数据库   │
└──────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                        设备层                                     │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐            │
│  │  PLC    │  │  SCADA  │  │  MES终端 │  │ 传感器   │            │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘            │
└─────────────────────────────────────────────────────────────────┘
```

## 7. 附录

### 7.1 状态码定义

| 状态码 | 说明 |
|-------|------|
| 200 | 成功 |
| 400 | 请求参数错误 |
| 401 | 未授权 |
| 403 | 禁止访问 |
| 404 | 资源不存在 |
| 500 | 服务器内部错误 |

### 7.2 工单状态流转

```
pending(待排程) → scheduled(已排程) → in_progress(生产中) → completed(已完成)
                                        │
                                        ▼
                                   suspended(暂停)
                                        │
                                        ▼
                                   closed(关闭)
```

### 7.3 设备状态定义

| 状态 | 说明 |
|-----|------|
| running | 运行中 |
| idle | 待机 |
| maintenance | 维护中 |
| fault | 故障 |
| offline | 离线 |
