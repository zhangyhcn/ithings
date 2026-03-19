# 物模型设计指南（融合 EdgeX Device Profile & Driver）
设计物模型（Thing Model）是物联网（IoT）系统中**标准化、结构化描述设备能力和行为**的核心环节，本质是为物理设备建立一套“数字孪生”的抽象模型；而在 EdgeX Foundry 框架中，**Device Profile** 是物模型在“设备驱动（Driver）层”的具体落地载体，是驱动与设备、EdgeX 核心服务交互的核心配置文件。

## 一、物模型设计的核心原则（适配 EdgeX 扩展）
1. **标准化**：字段命名、数据类型、单位等既遵循 IoT 通用规范，也对齐 EdgeX Device Profile 标准（如属性名匹配 Resource/Command 标识符）；
2. **最小必要**：只定义设备实际具备的能力，Driver 仅实现 Profile 中声明的资源，避免冗余；
3. **可扩展**：物模型版本与 Profile 版本同步迭代，支持 Driver 固件升级后新增 Resource/Command；
4. **易映射**：物模型的属性/服务/事件需能直接映射到 Profile 的 Resource/Command，降低 Driver 开发成本；
5. **分层抽象**：物模型聚焦“设备能力定义”，Profile 聚焦“Driver 如何读写/调用这些能力”，分工明确。

## 二、物模型与 EdgeX Device Profile/Driver 的核心映射关系
EdgeX 的 Device Profile 是**驱动（Driver）识别、操作设备的“配置契约”**，物模型是上层业务对设备的抽象，二者需严格一一映射，Driver 则是实现“Profile 定义的操作”与物理设备通信的核心组件。

### 1. 核心概念对应关系
| 物模型模块       | EdgeX Device Profile 核心元素 | EdgeX Driver 职责                                                                 |
|------------------|-------------------------------|----------------------------------------------------------------------------------|
| 属性（Property） | Resource（资源）+ Reading（读数） | 实现 Resource 的 `Read` 操作（只读属性）/ `Write` 操作（可写属性），将设备数据标准化为 EdgeX 读数格式 |
| 服务（Service）  | Command（命令）                | 实现 Command 的 `Get`/`Set` 逻辑，调用设备底层接口执行操作（如重启、校准），返回操作结果                |
| 事件（Event）    | Event（事件）+ Notification    | 监听设备上报的异常（如温度超限），封装为 EdgeX Event 推送到核心服务，或由 Driver 主动触发 Event 上报    |

### 2. EdgeX Device Profile 核心结构（与物模型对齐）
Device Profile 采用 YAML 格式，核心包含 `DeviceResources`（映射物模型属性）、`DeviceCommands`（映射物模型服务）、`CoreCommands`（EdgeX 核心服务交互配置），Driver 需严格按 Profile 定义实现接口。

## 三、物模型设计的具体步骤（融合 EdgeX Driver/Profile）
### 步骤1：梳理设备能力（需求调研）
以温湿度传感器为例，核心能力同前，新增“EdgeX Driver 适配考量”：
- 明确设备通信协议（如 Modbus、MQTT、HTTP），决定 Driver 类型（如 modbus-driver、mqtt-driver）；
- 明确属性的读写方式（如 Modbus 寄存器地址、MQTT 上报主题），为 Profile 定义 Resource 提供依据；
- 明确服务的触发方式（如串口指令、REST API），为 Profile 定义 Command 提供依据。

### 步骤2：结构化定义物模型（标准化）
物模型字段新增“EdgeX 映射字段”，确保与 Profile 无缝对接：

#### 1. 属性定义（新增 EdgeX 映射）
| 标识符          | 名称       | 数据类型 | 单位 | 读写权限 | 取值范围  | EdgeX 映射（Profile）                          | 描述                     |
|-----------------|------------|----------|------|----------|-----------|------------------------------------------------|--------------------------|
| device_model    | 设备型号   | string   | -    | R        | -         | Resource: device-model, 读取方式：静态配置     | 温湿度传感器型号，如TH100 |
| firmware_version| 固件版本   | string   | -    | R        | -         | Resource: firmware-version, 读取方式：设备寄存器0x01 | 设备固件版本，如V1.2.0   |
| temperature     | 实时温度   | float    | ℃    | R        | -40~85    | Resource: temperature, 读取方式：Modbus 0x02寄存器，缩放因子×0.1 | 设备采集的实时温度值     |
| humidity        | 实时湿度   | float    | %RH  | R        | 0~100     | Resource: humidity, 读取方式：Modbus 0x03寄存器，缩放因子×0.1 | 设备采集的实时湿度值     |
| sample_frequency| 采样频率   | int      | s    | RW       | 5~300     | Resource: sample-frequency, 读写方式：Modbus 0x04寄存器 | 温湿度采样间隔，默认10s  |

#### 2. 服务定义（新增 EdgeX 映射）
| 标识符          | 名称       | 入参                          | 出参                  | EdgeX 映射（Profile）                          | 描述                     |
|-----------------|------------|-------------------------------|-----------------------|------------------------------------------------|--------------------------|
| reboot          | 重启设备   | -                             | result（bool）        | Command: reboot, Driver 调用设备 0x05 功能码指令 | 触发设备重启，重启后5s上线 |
| calibrate       | 校准温湿度 | temp_offset（float）、hum_offset（float） | result（bool）、msg（string） | Command: calibrate, Driver 写入偏移值到 0x06/0x07 寄存器 | 校准温湿度采集偏差       |
| set_sample_freq | 设置采样频率 | freq（int）                   | result（bool）、msg（string） | Command: set-sample-frequency, Driver 写入值到 0x04 寄存器 | 修改设备采样间隔         |

#### 3. 事件定义（新增 EdgeX 映射）
| 标识符          | 名称       | 级别   | 携带数据                          | EdgeX 映射（Driver）                          | 描述                     |
|-----------------|------------|--------|-----------------------------------|------------------------------------------------|--------------------------|
| temp_overlimit  | 温度超限   | 警告   | current_temp、threshold           | Driver 监听 0x08 寄存器状态，触发 EdgeX Event 上报 | 实时温度超出设定阈值     |
| low_battery     | 低电量     | 警告   | battery_level                     | Driver 轮询 0x09 寄存器，电量<20%触发 Event    | 设备电量不足             |

### 步骤3：编写 EdgeX Device Profile（Driver 核心配置）
基于物模型生成适配 EdgeX 的 Device Profile YAML 文件，Driver 会加载该文件识别设备能力：
```yaml
# device-profile-th-sensor.yaml
name: "thermohygrometer-sensor"
manufacturer: "IoT Vendor"
model: "TH100"
labels:
  - "environment"
  - "sensor"
description: "温湿度传感器设备配置文件"
deviceResources:
  # 映射物模型属性：实时温度
  - name: "temperature"
    isHidden: false
    description: "实时温度值"
    properties:
      valueType: "Float32"
      readWrite: "R"
      units: "°C"
      defaultValue: "0.0"
      # Driver 需实现的底层配置（Modbus 示例）
      attributes: { "register": "0x02", "functionCode": "3", "scale": "0.1", "offset": "0" }
  # 映射物模型属性：采样频率（可写）
  - name: "sample-frequency"
    isHidden: false
    description: "温湿度采样间隔"
    properties:
      valueType: "Int32"
      readWrite: "RW"
      units: "s"
      defaultValue: "10"
      attributes: { "register": "0x04", "functionCode": "3", "writeFunctionCode": "6" }
# 映射物模型服务：命令定义
deviceCommands:
  # 映射“设置采样频率”服务
  - name: "set-sample-frequency"
    isHidden: false
    description: "修改设备采样间隔"
    resourceOperations:
      - { deviceResource: "sample-frequency", operation: "SET" }
  # 映射“重启设备”服务
  - name: "reboot"
    isHidden: false
    description: "重启温湿度传感器"
    resourceOperations:
      - { deviceResource: "reboot-trigger", operation: "SET" }
# CoreCommands：EdgeX 核心服务交互配置
coreCommands:
  - name: "temperature"
    get:
      path: "/api/v2/device/name/{deviceName}/temperature"
      responses:
        - code: "200"
          description: "Success"
          expectedValues: [ "temperature" ]
  - name: "set-sample-frequency"
    set:
      path: "/api/v2/device/name/{deviceName}/set-sample-frequency"
      parameters:
        - name: "sample-frequency"
          description: "采样频率（5-300s）"
          type: "Int32"