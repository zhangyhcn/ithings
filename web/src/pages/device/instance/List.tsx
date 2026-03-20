import { useState, useEffect } from 'react';
import { ProTable, ProColumns } from '@ant-design/pro-components';
import { Button, Modal, Form, Input, message, Space, Popconfirm, Select } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { deviceInstanceApi, deviceApi, nodeApi } from '@/services/api';
import type { DeviceInstance, Device, Node } from '@/types';

export default function DeviceInstanceList() {
  const [tableData, setTableData] = useState<DeviceInstance[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<DeviceInstance | null>(null);
  const [tenantId, setTenantId] = useState<string>('');
  const [siteId, setSiteId] = useState<string>('');
  const [form] = Form.useForm();
  const [devices, setDevices] = useState<Device[]>([]);
  const [nodes, setNodes] = useState<Node[]>([]);

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
      const [deviceData, nodeData] = await Promise.all([
        deviceApi.list(user.tenant_id),
        nodeApi.list(user.tenant_id),
      ]);
      setDevices(Array.isArray(deviceData) ? deviceData : []);
      setNodes(Array.isArray(nodeData) ? nodeData : []);

      if (!user.tenant_id || !siteId) {
        setTableData([]);
      } else {
        const data = await deviceInstanceApi.list(user.tenant_id, '', siteId);
        setTableData(Array.isArray(data.list) ? data.list : []);
      }
    } catch (error) {
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, [siteId]);

  const handleAdd = async (values: any) => {
    if (!tenantId || !siteId) return;
    try {
      if (editingRecord) {
        await deviceInstanceApi.update(tenantId, '', siteId, editingRecord.id, values);
        message.success('更新成功');
      } else {
        await deviceInstanceApi.create(tenantId, siteId, values);
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

  const handleEdit = (record: DeviceInstance) => {
    setEditingRecord(record);
    form.setFieldsValue(record);
    setModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    if (!tenantId || !siteId) return;
    try {
      await deviceInstanceApi.delete(tenantId, '', siteId, id);
      message.success('删除成功');
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const columns: ProColumns<DeviceInstance>[] = [
    {
      title: '设备名称',
      dataIndex: 'device_name',
      key: 'device_name',
      render: (_: any, record: DeviceInstance) => {
        const device = devices.find(d => d.id === record.device_id);
        return device ? device.name : record.device_id;
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
        }}
        onOk={form.submit}
      >
        <Form form={form} layout="vertical" onFinish={handleAdd}>
          <Form.Item
            name="device_id"
            label="选择设备"
            rules={[{ required: true, message: '请选择设备' }]}
          >
            <Select
              placeholder="请选择设备"
              showSearch
              optionFilterProp="children"
            >
              {devices.map((device) => (
                <Select.Option key={device.id} value={device.id}>
                  {device.name} {device.model ? `(${device.model})` : ''}
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item
            name="poll_interval_ms"
            label="轮询间隔(ms)"
            initialValue={1000}
            rules={[{ required: true, message: '请输入轮询间隔' }]}
          >
            <Input type="number" placeholder="请输入轮询间隔" />
          </Form.Item>
          <Form.Item
            name="node_id"
            label="部署节点"
            rules={[{ required: true, message: '请选择节点' }]}
          >
            <Select
              placeholder="请选择节点"
              showSearch
              optionFilterProp="children"
            >
              {nodes.map((node) => (
                <Select.Option key={node.id} value={node.id}>
                  {node.name}
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
}
