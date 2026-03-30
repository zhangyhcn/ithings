import React, { useState, useEffect } from 'react';
import { Table, Button, Space, Modal, Form, Input, Select, message, Popconfirm, Card, Alert, Spin } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { scmApi } from '@/services/api';
import { useScmOrg } from '@/hooks/useScmOrg';

interface Warehouse {
  id: string;
  tenant_id: string;
  org_id: string;
  code: string;
  name: string;
  warehouse_type: string;
  address?: string;
  manager?: string;
  phone?: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export default function WarehouseList() {
  const [warehouses, setWarehouses] = useState<Warehouse[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingWarehouse, setEditingWarehouse] = useState<Warehouse | null>(null);
  const [form] = Form.useForm();

  const { orgId, tenantId, loading: orgLoading, error: orgError } = useScmOrg();

  useEffect(() => {
    if (orgId) {
      loadWarehouses();
    }
  }, [orgId]);

  const loadWarehouses = async () => {
    setLoading(true);
    try {
      const data = await scmApi.listWarehouses(tenantId, orgId);
      setWarehouses(data);
    } catch (error) {
      message.error('加载仓库列表失败');
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = () => {
    setEditingWarehouse(null);
    form.resetFields();
    setModalVisible(true);
  };

  const handleEdit = (warehouse: Warehouse) => {
    setEditingWarehouse(warehouse);
    form.setFieldsValue(warehouse);
    setModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    try {
      await scmApi.deleteWarehouse(tenantId, orgId, id);
      message.success('删除成功');
      loadWarehouses();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      if (editingWarehouse) {
        await scmApi.updateWarehouse(tenantId, orgId, editingWarehouse.id, values);
        message.success('更新成功');
      } else {
        await scmApi.createWarehouse(tenantId, orgId, values);
        message.success('创建成功');
      }
      setModalVisible(false);
      loadWarehouses();
    } catch (error) {
      message.error('操作失败');
    }
  };

  const columns = [
    { title: '仓库编码', dataIndex: 'code', key: 'code' },
    { title: '仓库名称', dataIndex: 'name', key: 'name' },
    { title: '类型', dataIndex: 'warehouse_type', key: 'warehouse_type' },
    { title: '地址', dataIndex: 'address', key: 'address' },
    { title: '管理员', dataIndex: 'manager', key: 'manager' },
    { title: '电话', dataIndex: 'phone', key: 'phone' },
    { title: '状态', dataIndex: 'status', key: 'status' },
    {
      title: '操作',
      key: 'action',
      render: (_: any, record: Warehouse) => (
        <Space>
          <Button type="link" icon={<EditOutlined />} onClick={() => handleEdit(record)}>
            编辑
          </Button>
          <Popconfirm title="确认删除?" onConfirm={() => handleDelete(record.id)}>
            <Button type="link" danger icon={<DeleteOutlined />}>
              删除
            </Button>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <Card>
      {orgLoading && (
        <Spin tip="加载组织信息..." />
      )}
      {orgError && (
        <Alert
          message="提示"
          description={orgError}
          type="warning"
          showIcon
          style={{ marginBottom: 16 }}
        />
      )}
      {!orgLoading && !orgError && orgId && (
        <>
          <Space style={{ marginBottom: 16 }}>
            <Button type="primary" icon={<PlusOutlined />} onClick={handleCreate}>
              新增仓库
            </Button>
          </Space>
          <Table columns={columns} dataSource={warehouses} rowKey="id" loading={loading} />
        </>
      )}
      <Modal
        title={editingWarehouse ? '编辑仓库' : '新增仓库'}
        open={modalVisible}
        onOk={handleSubmit}
        onCancel={() => setModalVisible(false)}
      >
        <Form form={form} layout="vertical">
          <Form.Item name="code" label="仓库编码" rules={[{ required: true }]}>
            <Input disabled={!!editingWarehouse} />
          </Form.Item>
          <Form.Item name="name" label="仓库名称" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="warehouse_type" label="仓库类型" initialValue="normal">
            <Select>
              <Select.Option value="normal">普通仓库</Select.Option>
              <Select.Option value="cold">冷库</Select.Option>
              <Select.Option value="hazardous">危险品库</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="address" label="地址">
            <Input />
          </Form.Item>
          <Form.Item name="manager" label="管理员">
            <Input />
          </Form.Item>
          <Form.Item name="phone" label="电话">
            <Input />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}
