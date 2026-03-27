import { useState, useEffect } from 'react';
import { ProTable, ProColumns } from '@ant-design/pro-components';
import { Button, Modal, Form, Input, message, Space, Popconfirm, Select, InputNumber } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { deviceInstanceApi, deviceGroupApi, deviceApi, driverApi } from '@/services/api';
import type { DeviceInstance, DeviceGroup, Device, Driver } from '@/types';

export default function DeviceInstanceList() {
  const [tableData, setTableData] = useState<DeviceInstance[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<DeviceInstance | null>(null);
  const [tenantId, setTenantId] = useState<string>('');
  const [form] = Form.useForm();
  const [groups, setGroups] = useState<DeviceGroup[]>([]);
  const [devices, setDevices] = useState<Device[]>([]);
  const [drivers, setDrivers] = useState<Driver[]>([]);
  const [selectedGroup, setSelectedGroup] = useState<DeviceGroup | null>(null);
  const [selectedDevice, setSelectedDevice] = useState<Device | null>(null);
  const [selectedDriver, setSelectedDriver] = useState<Driver | null>(null);

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
      const [groupData, deviceData, driverData, instanceData] = await Promise.all([
        deviceGroupApi.list(user.tenant_id),
        deviceApi.list(user.tenant_id),
        driverApi.list(user.tenant_id),
        deviceInstanceApi.list(user.tenant_id),
      ]);
      setGroups(Array.isArray(groupData) ? groupData : []);
      setDevices(Array.isArray(deviceData) ? deviceData : []);
      setDrivers(Array.isArray(driverData) ? driverData : []);
      setTableData(Array.isArray(instanceData) ? instanceData : []);
    } catch (error) {
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  const handleAdd = async (values: any) => {
    if (!tenantId) return;
    try {
      if (editingRecord) {
        await deviceInstanceApi.update(tenantId, editingRecord.id, values);
        message.success('更新成功');
      } else {
        await deviceInstanceApi.create(tenantId, values);
        message.success('创建成功');
      }
      setModalVisible(false);
      form.resetFields();
      setEditingRecord(null);
      setSelectedGroup(null);
      setSelectedDevice(null);
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const handleEdit = (record: DeviceInstance) => {
    setEditingRecord(record);
    form.setFieldsValue({
      ...record,
      node_id: record.node_id || undefined,
    });
    const group = groups.find(g => g.id === record.group_id);
    setSelectedGroup(group || null);
    const device = devices.find(d => d.id === record.device_id);
    setSelectedDevice(device || null);
    setModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    if (!tenantId) return;
    try {
      await deviceInstanceApi.delete(tenantId, id);
      message.success('删除成功');
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const columns: ProColumns<DeviceInstance>[] = [
    {
      title: '设备名称',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: '所属设备组',
      dataIndex: 'group_id',
      key: 'group_id',
      render: (groupId: string) => {
        const group = groups.find(g => g.id === groupId);
        return group ? group.name : groupId;
      },
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
            onClick={() => handleEdit(record)}
          >
            编辑
          </Button>
          <Popconfirm
            title="确认删除?"
            onConfirm={() => handleDelete(record.id)}
          >
            <Button type="link" danger icon={<DeleteOutlined />}>
              删除
            </Button>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <div>
      <ProTable
        headerTitle="设备实例列表"
        columns={columns}
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
            创建设备实例
          </Button>,
        ]}
      />

      <Modal
        title={editingRecord ? '编辑设备实例' : '创建设备实例'}
        open={modalVisible}
        onCancel={() => {
          setModalVisible(false);
          form.resetFields();
          setEditingRecord(null);
          setSelectedGroup(null);
          setSelectedDevice(null);
          setSelectedDriver(null);
        }}
        onOk={form.submit}
        width={800}
      >
        <Form form={form} layout="vertical" onFinish={handleAdd}>
          <Form.Item
            name="group_id"
            label="所属设备组"
            rules={[{ required: true, message: '请选择设备组' }]}
          >
            <Select
              placeholder="请选择设备组"
              showSearch
              optionFilterProp="children"
              onChange={(value) => {
                const group = groups.find(g => g.id === value);
                setSelectedGroup(group || null);
              }}
            >
              {groups.map((group) => (
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
                  form.setFieldsValue({
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
            name="node_id"
            label="部署节点"
          >
            <Select
              placeholder="请选择节点（可选）"
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
            name="driver_config"
            label="驱动配置"
            extra="完整的驱动配置JSON，包含driver_name, driver_type, zmq, logging等，可以根据需要修改地址等参数"
            rules={[{ required: true, message: '请输入驱动配置' }]}
          >
            <Input.TextArea 
              rows={12} 
              placeholder="驱动配置JSON"
            />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
}
