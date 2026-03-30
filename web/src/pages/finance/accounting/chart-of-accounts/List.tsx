import React, { useEffect, useState } from 'react';
import { Card, Table, Button, Space, message, Modal, Form, Input, Select, Spin, Alert } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { useScmOrg } from '@/hooks/useScmOrg';

interface Account {
  id: string;
  account_code: string;
  account_name: string;
  account_type: string;
  parent_id?: string;
  level: number;
  is_leaf: boolean;
  debit_credit: string;
  currency: string;
  status: string;
  remarks?: string;
  created_at: string;
}

export default function ChartOfAccountsList() {
  const [accounts, setAccounts] = useState<Account[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [form] = Form.useForm();
  
  const { orgId, tenantId, loading: orgLoading, error: orgError } = useScmOrg();

  useEffect(() => {
    if (tenantId && orgId) {
      loadAccounts();
    }
  }, [tenantId, orgId]);

  const loadAccounts = async () => {
    if (!tenantId || !orgId) return;
    
    setLoading(true);
    try {
      const response = await fetch(`/api/v1/finance/accounts?tenant_id=${tenantId}&org_id=${orgId}`);
      const data = await response.json();
      setAccounts(data);
    } catch (error) {
      message.error('加载科目列表失败');
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = () => {
    form.resetFields();
    setModalVisible(true);
  };

  const handleSubmit = async () => {
    if (!tenantId || !orgId) {
      message.error('缺少租户或组织信息');
      return;
    }
    
    try {
      const values = await form.validateFields();
      const response = await fetch('/api/v1/finance/accounts', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          ...values,
          tenant_id: tenantId,
          org_id: orgId,
        }),
      });

      if (response.ok) {
        message.success('创建成功');
        setModalVisible(false);
        loadAccounts();
      } else {
        message.error('创建失败');
      }
    } catch (error) {
      message.error('提交失败');
    }
  };

  const handleDelete = async (id: string) => {
    Modal.confirm({
      title: '确认删除',
      content: '确定要删除这个会计科目吗？',
      onOk: async () => {
        try {
          const response = await fetch(`/api/v1/finance/accounts/${id}`, {
            method: 'DELETE',
          });

          if (response.ok) {
            message.success('删除成功');
            loadAccounts();
          } else {
            message.error('删除失败');
          }
        } catch (error) {
          message.error('删除失败');
        }
      },
    });
  };

  const columns: ColumnsType<Account> = [
    {
      title: '科目代码',
      dataIndex: 'account_code',
      key: 'account_code',
      width: 150,
    },
    {
      title: '科目名称',
      dataIndex: 'account_name',
      key: 'account_name',
      width: 200,
    },
    {
      title: '科目类型',
      dataIndex: 'account_type',
      key: 'account_type',
      width: 120,
      render: (type) => {
        const typeMap: Record<string, string> = {
          asset: '资产',
          liability: '负债',
          equity: '权益',
          income: '收入',
          expense: '费用',
        };
        return typeMap[type] || type;
      },
    },
    {
      title: '借/贷',
      dataIndex: 'debit_credit',
      key: 'debit_credit',
      width: 80,
      render: (dc) => dc === 'debit' ? '借' : '贷',
    },
    {
      title: '币种',
      dataIndex: 'currency',
      key: 'currency',
      width: 80,
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      width: 80,
      render: (status) => status === 'active' ? '启用' : '停用',
    },
    {
      title: '操作',
      key: 'action',
      width: 150,
      render: (_, record) => (
        <Space>
          <Button
            type="link"
            icon={<EditOutlined />}
            size="small"
          >
            编辑
          </Button>
          <Button
            type="link"
            danger
            icon={<DeleteOutlined />}
            size="small"
            onClick={() => handleDelete(record.id)}
          >
            删除
          </Button>
        </Space>
      ),
    },
  ];

  return (
    <Card
      title="会计科目管理"
      extra={
        <Button
          type="primary"
          icon={<PlusOutlined />}
          onClick={handleCreate}
          disabled={orgLoading || !!orgError}
        >
          新增科目
        </Button>
      }
    >
      {orgLoading ? (
        <Spin tip="加载组织信息..." />
      ) : orgError ? (
        <Alert message={orgError} type="error" showIcon />
      ) : (
        <Table
          columns={columns}
          dataSource={accounts}
          rowKey="id"
          loading={loading}
          pagination={{
            pageSize: 20,
            showSizeChanger: true,
            showTotal: (total) => `共 ${total} 条`,
          }}
        />
      )}

      <Modal
        title="新增会计科目"
        open={modalVisible}
        onOk={handleSubmit}
        onCancel={() => setModalVisible(false)}
        width={600}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            name="account_code"
            label="科目代码"
            rules={[{ required: true, message: '请输入科目代码' }]}
          >
            <Input placeholder="如: 1001" />
          </Form.Item>

          <Form.Item
            name="account_name"
            label="科目名称"
            rules={[{ required: true, message: '请输入科目名称' }]}
          >
            <Input placeholder="如: 库存现金" />
          </Form.Item>

          <Form.Item
            name="account_type"
            label="科目类型"
            rules={[{ required: true, message: '请选择科目类型' }]}
          >
            <Select placeholder="请选择">
              <Select.Option value="asset">资产</Select.Option>
              <Select.Option value="liability">负债</Select.Option>
              <Select.Option value="equity">权益</Select.Option>
              <Select.Option value="income">收入</Select.Option>
              <Select.Option value="expense">费用</Select.Option>
            </Select>
          </Form.Item>

          <Form.Item
            name="level"
            label="科目级别"
            rules={[{ required: true, message: '请输入科目级别' }]}
          >
            <Input type="number" placeholder="如: 1" />
          </Form.Item>

          <Form.Item
            name="is_leaf"
            label="是否末级科目"
          >
            <Select placeholder="请选择">
              <Select.Option value={true}>是</Select.Option>
              <Select.Option value={false}>否</Select.Option>
            </Select>
          </Form.Item>

          <Form.Item
            name="debit_credit"
            label="借/贷方向"
            rules={[{ required: true, message: '请选择借/贷方向' }]}
          >
            <Select placeholder="请选择">
              <Select.Option value="debit">借</Select.Option>
              <Select.Option value="credit">贷</Select.Option>
            </Select>
          </Form.Item>

          <Form.Item
            name="remarks"
            label="备注"
          >
            <Input.TextArea rows={3} placeholder="请输入备注" />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}
