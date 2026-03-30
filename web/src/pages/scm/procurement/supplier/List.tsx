import React, { useState, useEffect } from 'react';
import { Table, Button, Space, Modal, Form, Input, message, Popconfirm, Card, Alert, Spin } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { scmApi } from '@/services/api';
import { useScmOrg } from '@/hooks/useScmOrg';

interface Supplier {
  id: string;
  tenant_id: string;
  org_id: string;
  supplier_code: string;
  supplier_name: string;
  contact_person?: string;
  contact_phone?: string;
  contact_email?: string;
  address?: string;
  bank_name?: string;
  bank_account?: string;
  tax_number?: string;
  supplier_type?: string;
  credit_level?: string;
  remarks?: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export default function SupplierList() {
  const [suppliers, setSuppliers] = useState<Supplier[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingSupplier, setEditingSupplier] = useState<Supplier | null>(null);
  const [form] = Form.useForm();

  const { orgId, tenantId, loading: orgLoading, error: orgError } = useScmOrg();

  useEffect(() => {
    if (orgId) {
      loadSuppliers();
    }
  }, [orgId]);

  const loadSuppliers = async () => {
    setLoading(true);
    try {
      const data = await scmApi.listSuppliers(tenantId, orgId);
      setSuppliers(data);
    } catch (error) {
      message.error('加载供应商列表失败');
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = () => {
    setEditingSupplier(null);
    form.resetFields();
    setModalVisible(true);
  };

  const handleEdit = (supplier: Supplier) => {
    setEditingSupplier(supplier);
    form.setFieldsValue(supplier);
    setModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    try {
      await scmApi.deleteSupplier(tenantId, orgId, id);
      message.success('删除成功');
      loadSuppliers();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      if (editingSupplier) {
        await scmApi.updateSupplier(tenantId, orgId, editingSupplier.id, values);
        message.success('更新成功');
      } else {
        await scmApi.createSupplier(tenantId, orgId, values);
        message.success('创建成功');
      }
      setModalVisible(false);
      loadSuppliers();
    } catch (error) {
      message.error('操作失败');
    }
  };

  const columns = [
    {
      title: '供应商编码',
      dataIndex: 'supplier_code',
      key: 'supplier_code',
    },
    {
      title: '供应商名称',
      dataIndex: 'supplier_name',
      key: 'supplier_name',
    },
    {
      title: '联系人',
      dataIndex: 'contact_person',
      key: 'contact_person',
    },
    {
      title: '联系电话',
      dataIndex: 'contact_phone',
      key: 'contact_phone',
    },
    {
      title: '联系邮箱',
      dataIndex: 'contact_email',
      key: 'contact_email',
    },
    {
      title: '供应商类型',
      dataIndex: 'supplier_type',
      key: 'supplier_type',
    },
    {
      title: '信用等级',
      dataIndex: 'credit_level',
      key: 'credit_level',
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => (
        <span style={{ color: status === 'active' ? 'green' : 'red' }}>
          {status === 'active' ? '活跃' : '停用'}
        </span>
      ),
    },
    {
      title: '操作',
      key: 'action',
      render: (_: any, record: Supplier) => (
        <Space>
          <Button
            type="link"
            icon={<EditOutlined />}
            onClick={() => handleEdit(record)}
          >
            编辑
          </Button>
          <Popconfirm
            title="确定要删除吗？"
            onConfirm={() => handleDelete(record.id)}
            okText="确定"
            cancelText="取消"
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
    <Card>
      <div style={{ marginBottom: 16 }}>
        <Button type="primary" icon={<PlusOutlined />} onClick={handleCreate}>
          新增供应商
        </Button>
      </div>
      <Table
        columns={columns}
        dataSource={suppliers}
        rowKey="id"
        loading={loading}
      />
      <Modal
        title={editingSupplier ? '编辑供应商' : '新增供应商'}
        open={modalVisible}
        onOk={handleSubmit}
        onCancel={() => setModalVisible(false)}
        width={600}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            name="supplier_code"
            label="供应商编码"
            rules={[{ required: true, message: '请输入供应商编码' }]}
          >
            <Input disabled={!!editingSupplier} />
          </Form.Item>
          <Form.Item
            name="supplier_name"
            label="供应商名称"
            rules={[{ required: true, message: '请输入供应商名称' }]}
          >
            <Input />
          </Form.Item>
          <Form.Item name="contact_person" label="联系人">
            <Input />
          </Form.Item>
          <Form.Item name="contact_phone" label="联系电话">
            <Input />
          </Form.Item>
          <Form.Item name="contact_email" label="联系邮箱">
            <Input />
          </Form.Item>
          <Form.Item name="address" label="地址">
            <Input />
          </Form.Item>
          <Form.Item name="bank_name" label="银行名称">
            <Input />
          </Form.Item>
          <Form.Item name="bank_account" label="银行账号">
            <Input />
          </Form.Item>
          <Form.Item name="tax_number" label="税号">
            <Input />
          </Form.Item>
          <Form.Item name="supplier_type" label="供应商类型">
            <Input />
          </Form.Item>
          <Form.Item name="credit_level" label="信用等级">
            <Input />
          </Form.Item>
          <Form.Item name="remarks" label="备注">
            <Input.TextArea />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}
