import { useState, useEffect } from 'react';
import { ProTable, ProColumns } from '@ant-design/pro-components';
import { Button, Modal, Form, Input, message, Space, Popconfirm } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { deviceInstanceApi } from '@/services/api';
import type { DeviceInstance } from '@/types';

export default function DeviceInstanceList() {
  const [tableData, setTableData] = useState<DeviceInstance[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<DeviceInstance | null>(null);
  const [tenantId, setTenantId] = useState<string>('');
  const [siteId, setSiteId] = useState<string>('');
  const [form] = Form.useForm();

  const fetchData = async () => {
    const userStr = localStorage.getItem('user');
    if (!userStr) {
      setTableData([]);
      setLoading(false);
      return;
    }
    const user = JSON.parse(userStr);
    setTenantId(user.tenant_id);
    
    if (!user.tenant_id || !siteId) {
      setTableData([]);
      setLoading(false);
      return;
    }
    setLoading(true);
    try {
      const data = await deviceInstanceApi.list(user.tenant_id, '', siteId);
      setTableData(Array.isArray(data.list) ? data.list : []);
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
      title: '名称',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: '品牌型号',
      dataIndex: 'brand_model',
      key: 'brand_model',
    },
    {
      title: '轮询间隔',
      dataIndex: 'poll_interval_ms',
      key: 'poll_interval_ms',
      render: (val: number) => `${val} ms`,
    },
    {
      title: '设备类型',
      dataIndex: 'device_type',
      key: 'device_type',
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
            name="name"
            label="设备名称"
            rules={[{ required: true, message: '请输入设备名称' }]}
          >
            <Input placeholder="请输入设备名称" />
          </Form.Item>
          <Form.Item name="brand_model" label="品牌型号">
            <Input placeholder="请输入品牌型号" />
          </Form.Item>
          <Form.Item
            name="product_id"
            label="关联产品"
            rules={[{ required: true, message: '请选择产品' }]}
          >
            <Input placeholder="请输入产品 UUID" />
          </Form.Item>
          <Form.Item
            name="driver_id"
            label="关联驱动"
            rules={[{ required: true, message: '请选择驱动' }]}
          >
            <Input placeholder="请输入驱动 UUID" />
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
            name="device_type"
            label="设备类型"
            rules={[{ required: true, message: '请输入设备类型' }]}
          >
            <Input placeholder="如: electricity-meter" />
          </Form.Item>
          <Form.Item
            name="node_id"
            label="部署节点"
            rules={[{ required: true, message: '请选择节点' }]}
          >
            <Input placeholder="请输入节点 UUID" />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
}
