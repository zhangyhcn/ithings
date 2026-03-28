import { useState, useEffect } from 'react';
import { ProTable, ProColumns, ProRowSelection } from '@ant-design/pro-components';
import { Button, Modal, Form, Input, message, Space, Popconfirm, Select, InputNumber, Row, Col, Dropdown } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, DownOutlined, UpOutlined, CheckCircleOutlined } from '@ant-design/icons';
import { deviceGroupApi, organizationApi, siteApi, namespaceApi, productApi, driverApi, deviceInstanceApi, deviceApi, nodeApi } from '@/services/api';
import type { DeviceGroup, Organization, Site, Namespace, Device, Driver, DeviceInstance, Node } from '@/types';

export default function DeviceGroupList() {
  const [tableData, setTableData] = useState<DeviceGroup[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<DeviceGroup | null>(null);
  const [tenantId, setTenantId] = useState<string>('');
  const [form] = Form.useForm();
  const [organizations, setOrganizations] = useState<Organization[]>([]);
  const [selectedOrgId, setSelectedOrgId] = useState<string>('');
  const [sites, setSites] = useState<Site[]>([]);
  const [selectedSiteId, setSelectedSiteId] = useState<string>('');
  const [namespaces, setNamespaces] = useState<Namespace[]>([]);
  const [drivers, setDrivers] = useState<Driver[]>([]);
  const [devices, setDevices] = useState<Device[]>([]);
  const [nodes, setNodes] = useState<Node[]>([]);
  const [instances, setInstances] = useState<DeviceInstance[]>([]);
  const [instanceModalVisible, setInstanceModalVisible] = useState(false);
  const [editingInstance, setEditingInstance] = useState<DeviceInstance | null>(null);
  const [selectedGroup, setSelectedGroup] = useState<DeviceGroup | null>(null);
  const [selectedDevice, setSelectedDevice] = useState<Device | null>(null);
  const [selectedDriver, setSelectedDriver] = useState<Driver | null>(null);
  const [instanceForm] = Form.useForm();
  
  // 发布相关状态
  const [publishModalVisible, setPublishModalVisible] = useState(false);
  const [publishingGroup, setPublishingGroup] = useState<DeviceGroup | null>(null);
  const [selectedNode, setSelectedNode] = useState<Node | null>(null);
  const [nodeLabels, setNodeLabels] = useState<Record<string, string>>({});
  const [filteredNodes, setFilteredNodes] = useState<Node[]>([]);
  const [publishForm] = Form.useForm();
  const [labelInput, setLabelInput] = useState<string>('');

  const fetchData = async () => {
    const userStr = localStorage.getItem('user');
    if (!userStr) {
      setTableData([]);
      setLoading(false);
      return;
    }
    const user = JSON.parse(userStr);
    setTenantId(user.tenant_id);
    
    setLoading(true);
    try {
      // 获取组织、站点、命名空间用于表单下拉选择
      // 设备组列表已经包含关联名称，不需要前端再匹配
      const [orgData, productData, driverData, deviceData, nodeData, instanceData, groupData, namespaceData] = await Promise.all([
        organizationApi.list(user.tenant_id),
        productApi.list(user.tenant_id),
        driverApi.list(user.tenant_id),
        deviceApi.list(user.tenant_id),
        nodeApi.list(user.tenant_id),
        deviceInstanceApi.list(user.tenant_id),
        deviceGroupApi.list(user.tenant_id),
        namespaceApi.list(user.tenant_id),
      ]);
      setOrganizations(Array.isArray(orgData) ? orgData : []);
      setDrivers(Array.isArray(driverData) ? driverData : []);
      setDevices(Array.isArray(deviceData) ? deviceData : []);
      setNodes(Array.isArray(nodeData) ? nodeData : []);
      setInstances(Array.isArray(instanceData) ? instanceData : []);
      setTableData(Array.isArray(groupData) ? groupData : []);
      setNamespaces(Array.isArray(namespaceData) ? namespaceData : []);
    } catch (error) {
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  const fetchSites = async (orgId: string) => {
    if (!tenantId || !orgId) return;
    try {
      const siteData = await siteApi.list(tenantId);
      const orgSites = Array.isArray(siteData) 
        ? siteData.filter((s: Site) => s.organization_id === orgId)
        : [];
      setSites(orgSites);
    } catch (error) {
      console.error(error);
    }
  };

  const fetchNamespaces = async (siteId: string) => {
    if (!tenantId || !siteId) return;
    try {
      const namespaceData = await namespaceApi.list(tenantId);
      const siteNamespaces = Array.isArray(namespaceData) 
        ? namespaceData.filter((ns: Namespace) => ns.site_id === siteId)
        : [];
      setNamespaces(siteNamespaces);
    } catch (error) {
      console.error(error);
    }
  };

  const handleAdd = async (values: any) => {
    if (!tenantId) return;
    try {
      if (editingRecord) {
        await deviceGroupApi.update(tenantId, editingRecord.id, values);
        message.success('更新成功');
      } else {
        await deviceGroupApi.create(tenantId, values);
        message.success('创建成功');
      }
      setModalVisible(false);
      form.resetFields();
      setEditingRecord(null);
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const handleEdit = async (record: DeviceGroup) => {
    setEditingRecord(record);
    form.setFieldsValue({
      ...record,
      node_id: record.node_id || undefined,
    });
    setSelectedOrgId(record.org_id);
    setSelectedSiteId(record.site_id);
    
    if (record.org_id && tenantId) {
      try {
        const siteData = await siteApi.list(tenantId);
        const orgSites = Array.isArray(siteData) 
          ? siteData.filter((s: Site) => s.organization_id === record.org_id)
          : [];
        setSites(orgSites);
      } catch (error) {
        console.error(error);
      }
    }
    
    if (record.site_id && tenantId) {
      try {
        const namespaceData = await namespaceApi.list(tenantId);
        const siteNamespaces = Array.isArray(namespaceData) 
          ? namespaceData.filter((ns: Namespace) => ns.site_id === record.site_id)
          : [];
        setNamespaces(siteNamespaces);
      } catch (error) {
        console.error(error);
      }
    }
    
    setModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    if (!tenantId) return;
    try {
      await deviceGroupApi.delete(tenantId, id);
      message.success('删除成功');
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const handlePublish = (group: DeviceGroup) => {
    setPublishingGroup(group);
    setSelectedNode(group.node_id ? nodes.find(n => n.id === group.node_id) : null);
    setNodeLabels({});
    setFilteredNodes(nodes);
    setLabelInput('');
    publishForm.resetFields();
    if (group.node_id) {
      publishForm.setFieldsValue({ node_id: group.node_id });
    }
    setPublishModalVisible(true);
  };

  const handleLabelInputChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    const text = e.target.value;
    setLabelInput(text);
  };

  const handleLabelSearch = () => {
    // 解析输入的value（逗号分隔）
    const values = labelInput
      .split(',')
      .map(v => v.trim())
      .filter(v => v);
    
    // 根据value筛选节点
    const filtered = nodes.filter(node => {
      // 检查节点的任何标签值是否包含输入的value
      return Object.values(node.labels).some(labelValue => 
        values.some(value => labelValue.includes(value))
      );
    });
    setFilteredNodes(filtered);
  };

  const handlePublishSubmit = async (values: any) => {
    if (!tenantId || !publishingGroup) return;
    try {
      // 解析标签
      const labels: Record<string, string> = {};
      labelInput.split('\n').forEach(line => {
        const parts = line.split('=');
        if (parts.length === 2) {
          labels[parts[0].trim()] = parts[1].trim();
        }
      });
      
      await deviceGroupApi.publish(tenantId, publishingGroup.id, {
        ...values,
        labels
      });
      message.success('发布成功');
      setPublishModalVisible(false);
      setPublishingGroup(null);
      setSelectedNode(null);
      setLabelInput('');
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  // 设备实例相关操作
  const handleAddInstance = async (values: any) => {
    if (!tenantId) return;
    try {
      if (editingInstance) {
        await deviceInstanceApi.update(tenantId, editingInstance.id, values);
        message.success('更新成功');
      } else {
        await deviceInstanceApi.create(tenantId, values);
        message.success('创建成功');
      }
      setInstanceModalVisible(false);
      instanceForm.resetFields();
      setEditingInstance(null);
      setSelectedGroup(null);
      setSelectedDevice(null);
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const handleEditInstance = (record: DeviceInstance) => {
    setEditingInstance(record);
    instanceForm.setFieldsValue({
      ...record,
      node_id: record.node_id || undefined,
      driver_config: JSON.stringify(record.driver_config, null, 2),
    });
    const group = tableData.find(g => g.id === record.group_id);
    setSelectedGroup(group || null);
    const device = devices.find(d => d.id === record.device_id);
    setSelectedDevice(device || null);
    setInstanceModalVisible(true);
  };

  const handleDeleteInstance = async (id: string) => {
    if (!tenantId) return;
    try {
      await deviceInstanceApi.delete(tenantId, id);
      message.success('删除成功');
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const openInstanceModal = (group: DeviceGroup) => {
    setSelectedGroup(group);
    setEditingInstance(null);
    instanceForm.resetFields();
    instanceForm.setFieldsValue({ group_id: group.id });
    setInstanceModalVisible(true);
  };

  // 设备实例列定义
  const instanceColumns: ProColumns<DeviceInstance>[] = [
    {
      title: '设备名称',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: '设备定义',
      dataIndex: 'device_id',
      key: 'device_id',
      render: (deviceId: string) => {
        const device = devices.find(d => d.id === deviceId);
        return device ? device.name : deviceId;
      },
    },
    {
      title: '轮询间隔',
      dataIndex: 'poll_interval_ms',
      key: 'poll_interval_ms',
      render: (val: number) => `${val} ms`,
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => (
        <span style={{ color: status === 'running' ? 'green' : status === 'pending' ? 'orange' : 'red' }}>
          {
            {
              pending: '等待中',
              running: '运行中',
              stopped: '已停止',
              error: '错误',
            }[status] || status
          }
        </span>
      ),
    },
    {
      title: '创建时间',
      dataIndex: 'created_at',
      key: 'created_at',
      render: (text: string) => new Date(text).toLocaleString(),
    },
    {
      title: '操作',
      key: 'action',
      render: (_, record) => (
        <Space>
          <Button
            type="link"
            icon={<EditOutlined />}
            onClick={() => handleEditInstance(record)}
          >
            编辑
          </Button>
          <Popconfirm
            title="确认删除?"
            onConfirm={() => handleDeleteInstance(record.id)}
          >
            <Button type="link" danger icon={<DeleteOutlined />}>
              删除
            </Button>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  const columns: ProColumns<DeviceGroup>[] = [
    {
      title: '设备组名称',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: '所属组织',
      dataIndex: 'org_name',
      key: 'org_name',
      render: (text: string, record: DeviceGroup) => text || record.org_id,
    },
    {
      title: '所属站点',
      dataIndex: 'site_name',
      key: 'site_name',
      render: (text: string, record: DeviceGroup) => text || record.site_id,
    },
    {
      title: '所属空间',
      dataIndex: 'namespace_name',
      key: 'namespace_name',
      render: (text: string, record: DeviceGroup) => text || record.namespace_id || '-',
    },
    {
      title: '部署节点',
      dataIndex: 'node_id',
      key: 'node_id',
      render: (nodeId: string) => {
        if (!nodeId) return '-';
        const node = nodes.find(n => n.id === nodeId);
        return node ? node.name : nodeId;
      },
    },
    {
      title: '描述',
      dataIndex: 'description',
      key: 'description',
      ellipsis: true,
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => (
        <span style={{ color: status === 'active' || status === 'published' ? 'green' : 'red' }}>
          {
            {
              active: '启用',
              published: '已发布',
              disabled: '禁用',
            }[status] || status
          }
        </span>
      ),
    },
    {
      title: '创建时间',
      dataIndex: 'created_at',
      key: 'created_at',
      render: (text: string) => new Date(text).toLocaleString(),
    },
    {
      title: '操作',
      key: 'action',
      render: (_, record) => (
        <Dropdown menu={{
          items: [
            {
              key: 'addInstance',
              label: (
                <Button type="link" icon={<PlusOutlined />} onClick={() => openInstanceModal(record)}>
                  添加
                </Button>
              ),
            },
            {
              key: 'edit',
              label: (
                <Button type="link" icon={<EditOutlined />} onClick={() => handleEdit(record)}>
                  编辑
                </Button>
              ),
            },
            {
              key: 'publish',
              label: (
                <Button type="link" icon={<CheckCircleOutlined />} onClick={() => handlePublish(record)}>
                  发布
                </Button>
              ),
            },
            {
              key: 'delete',
              label: (
                <Popconfirm
                  title="确认删除?"
                  onConfirm={() => handleDelete(record.id)}
                >
                  <Button type="link" danger icon={<DeleteOutlined />}>
                    删除
                  </Button>
                </Popconfirm>
              ),
            },
          ],
        }}>
          <Button type="primary">
            操作 <DownOutlined />
          </Button>
        </Dropdown>
      ),
    },
  ];

  return (
    <div>
      <ProTable
        headerTitle="设备组列表"
        columns={columns}
        expandable={{
          expandedRowRender: (record) => {
            const groupInstances = instances.filter(instance => instance.group_id === record.id);
            return (
              <div style={{ padding: '16px' }}>
                <ProTable
                  headerTitle="设备实例列表"
                  columns={instanceColumns}
                  dataSource={groupInstances}
                  rowKey="id"
                  search={false}
                  pagination={{ showSizeChanger: false, pageSize: 5 }}
                />
              </div>
            );
          },
        }}
        dataSource={tableData}
        loading={loading}
        rowKey="id"
        search={false}
        toolBarRender={() => [
          <Button
            type="primary"
            key="add"
            icon={<PlusOutlined />}
            onClick={() => setModalVisible(true)}
          >
            创建设备组
          </Button>,
        ]}
      />

      <Modal
        title={editingRecord ? '编辑设备组' : '创建设备组'}
        open={modalVisible}
        onCancel={() => {
          setModalVisible(false);
          form.resetFields();
          setEditingRecord(null);
          setSelectedOrgId('');
          setSites([]);
        }}
        onOk={form.submit}
        width={600}
      >
        <Form form={form} layout="vertical" onFinish={handleAdd}>
          <Form.Item
            name="org_id"
            label="所属组织"
            rules={[{ required: true, message: '请选择组织' }]}
          >
            <Select
              placeholder="请选择组织"
              showSearch
              optionFilterProp="children"
              onChange={(value) => {
                setSelectedOrgId(value);
                form.setFieldsValue({ site_id: undefined });
                fetchSites(value);
              }}
            >
              {organizations.map((org) => (
                <Select.Option key={org.id} value={org.id}>
                  {org.name}
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item
            name="site_id"
            label="所属站点"
            rules={[{ required: true, message: '请选择站点' }]}
          >
            <Select
              placeholder="请选择站点"
              showSearch
              optionFilterProp="children"
              disabled={!selectedOrgId}
              onChange={(value) => {
                setSelectedSiteId(value);
                form.setFieldsValue({ namespace_id: undefined });
                fetchNamespaces(value);
              }}
            >
              {sites.map((site) => (
                <Select.Option key={site.id} value={site.id}>
                  {site.name}
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item
            name="namespace_id"
            label="所属空间"
            rules={[{ required: true, message: '请选择空间' }]}
          >
            <Select
              placeholder="请选择空间"
              showSearch
              optionFilterProp="children"
              disabled={!selectedSiteId}
            >
              {namespaces.map((ns) => (
                <Select.Option key={ns.id} value={ns.id}>
                  {ns.name}
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item
            name="name"
            label="设备组名称"
            rules={[{ required: true, message: '请输入设备组名称' }]}
          >
            <Input placeholder="请输入设备组名称" />
          </Form.Item>
          <Form.Item
            name="node_id"
            label="部署节点"
          >
            <Select
              placeholder="请选择部署节点（可选）"
              showSearch
              optionFilterProp="children"
              allowClear
            >
              {nodes.map((node) => (
                <Select.Option key={node.id} value={node.id}>
                  {node.name}
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item
            name="description"
            label="描述"
          >
            <Input.TextArea placeholder="请输入描述" rows={3} />
          </Form.Item>
        </Form>
      </Modal>

      <Modal
        title={editingInstance ? '编辑设备实例' : '添加设备实例'}
        open={instanceModalVisible}
        onCancel={() => {
          setInstanceModalVisible(false);
          instanceForm.resetFields();
          setEditingInstance(null);
          setSelectedGroup(null);
          setSelectedDevice(null);
          setSelectedDriver(null);
        }}
        onOk={instanceForm.submit}
        width={800}
      >
        <Form form={instanceForm} layout="vertical" onFinish={handleAddInstance}>
          <Form.Item
            name="group_id"
            label="所属设备组"
            rules={[{ required: true, message: '请选择设备组' }]}
          >
            <Select
              placeholder="请选择设备组"
              showSearch
              optionFilterProp="children"
              disabled={!!selectedGroup}
            >
              {tableData.map((group) => (
                <Select.Option key={group.id} value={group.id}>
                  {group.name}
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item
            name="device_id"
            label="选择设备定义"
            rules={[{ required: true, message: '请选择设备定义' }]}
          >
            <Select
              placeholder="请选择设备定义"
              showSearch
              optionFilterProp="children"
              onChange={(value) => {
                const device = devices.find(d => d.id === value);
                setSelectedDevice(device || null);
                if (device && device.device_profile) {
                  instanceForm.setFieldsValue({
                    driver_config: JSON.stringify(device.device_profile, null, 2),
                  });
                }
              }}
            >
              {devices.map((device) => (
                <Select.Option key={device.id} value={device.id}>
                  {device.name} {device.model ? `(${device.model})` : ''}
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item
            name="name"
            label="设备实例名称"
            rules={[{ required: true, message: '请输入设备实例名称' }]}
          >
            <Input placeholder="请输入设备实例名称" />
          </Form.Item>
          <Form.Item
            name="poll_interval_ms"
            label="轮询间隔(ms)"
            initialValue={1000}
            rules={[{ required: true, message: '请输入轮询间隔' }]}
          >
            <InputNumber min={100} placeholder="请输入轮询间隔" style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item
            name="driver_config"
            label="驱动配置"
            extra="完整的驱动配置JSON，包含driver_name, driver_type, zmq, logging等，可以根据需要修改地址等参数"
            rules={[{ required: true, message: '请输入驱动配置' }]}
          >
            <Input.TextArea 
              rows={10} 
              placeholder="驱动配置JSON"
            />
          </Form.Item>
        </Form>
      </Modal>

      <Modal
        title="发布设备组"
        open={publishModalVisible}
        onCancel={() => {
          setPublishModalVisible(false);
          setPublishingGroup(null);
          setSelectedNode(null);
          setNodeLabels({});
          setLabelInput('');
          publishForm.resetFields();
        }}
        onOk={publishForm.submit}
        width={600}
      >
        <Form form={publishForm} layout="vertical" onFinish={handlePublishSubmit}>
          <Form.Item
            label="节点标签筛选"
          >
            <Row gutter={16}>
              <Col span={18}>
                <Input.TextArea 
                  rows={3} 
                  placeholder="输入标签值，多个值用逗号隔开"
                  value={labelInput}
                  onChange={handleLabelInputChange}
                />
              </Col>
              <Col span={6}>
                <Button 
                  type="primary" 
                  onClick={handleLabelSearch}
                  style={{ marginTop: 8 }}
                >
                  搜索
                </Button>
              </Col>
            </Row>
          </Form.Item>
          <Form.Item
            name="node_id"
            label="选择节点"
            rules={[{ required: true, message: '请选择节点' }]}
          >
            <Select
              placeholder="请选择节点"
              showSearch
              optionFilterProp="children"
              onChange={(value) => {
                const node = filteredNodes.find(n => n.id === value);
                setSelectedNode(node || null);
              }}
            >
              {filteredNodes.map((node) => (
                <Select.Option key={node.id} value={node.id}>
                  {node.name} ({Object.entries(node.labels).map(([k, v]) => `${k}=${v}`).join(', ')})
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
}
