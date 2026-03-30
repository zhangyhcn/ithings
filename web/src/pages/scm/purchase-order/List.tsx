import React, { useState, useEffect } from 'react';
import { Table, Button, Space, Modal, Form, Input, InputNumber, DatePicker, message, Popconfirm, Card, Select } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { scmApi } from '@/services/api';
import { useScmOrg } from '@/hooks/useScmOrg';
import dayjs from 'dayjs';

interface PurchaseOrder {
  id: string;
  tenant_id: string;
  org_id: string;
  order_no: string;
  supplier_id: string;
  order_date: string;
  expected_delivery_date?: string;
  payment_terms?: string;
  delivery_address?: string;
  contact_person?: string;
  contact_phone?: string;
  total_amount: string;
  currency?: string;
  remarks?: string;
  status: string;
  created_by?: string;
  approved_by?: string;
  approved_at?: string;
  created_at: string;
  updated_at: string;
}

interface Supplier {
  id: string;
  supplier_name: string;
  supplier_code: string;
}

export default function PurchaseOrderList() {
  const [orders, setOrders] = useState<PurchaseOrder[]>([]);
  const [suppliers, setSuppliers] = useState<Supplier[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingOrder, setEditingOrder] = useState<PurchaseOrder | null>(null);
  const [form] = Form.useForm();

  const { orgId, tenantId, loading: orgLoading, error: orgError } = useScmOrg();

  useEffect(() => {
    if (orgId) {
      loadOrders();
      loadSuppliers();
    }
  }, [orgId]);

  const loadOrders = async () => {
    setLoading(true);
    try {
      const data = await scmApi.listPurchaseOrders(tenantId, orgId);
      setOrders(data);
    } catch (error) {
      message.error('加载采购订单列表失败');
    } finally {
      setLoading(false);
    }
  };

  const loadSuppliers = async () => {
    try {
      const data = await scmApi.listSuppliers(tenantId, orgId);
      setSuppliers(data);
    } catch (error) {
      message.error('加载供应商列表失败');
    }
  };

  const handleCreate = () => {
    setEditingOrder(null);
    form.resetFields();
    setModalVisible(true);
  };

  const handleEdit = (order: PurchaseOrder) => {
    setEditingOrder(order);
    form.setFieldsValue({
      ...order,
      order_date: dayjs(order.order_date),
      expected_delivery_date: order.expected_delivery_date ? dayjs(order.expected_delivery_date) : null,
      total_amount: parseFloat(order.total_amount),
    });
    setModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    try {
      await scmApi.deletePurchaseOrder(tenantId, orgId, id);
      message.success('删除成功');
      loadOrders();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      const submitData = {
        ...values,
        order_date: values.order_date.format('YYYY-MM-DD'),
        expected_delivery_date: values.expected_delivery_date?.format('YYYY-MM-DD'),
        supplier_id: values.supplier_id,
        total_amount: values.total_amount,
      };
      
      if (editingOrder) {
        await scmApi.updatePurchaseOrder(tenantId, orgId, editingOrder.id, submitData);
        message.success('更新成功');
      } else {
        await scmApi.createPurchaseOrder(tenantId, orgId, submitData);
        message.success('创建成功');
      }
      setModalVisible(false);
      loadOrders();
    } catch (error) {
      message.error('操作失败');
    }
  };

  const getStatusText = (status: string) => {
    const statusMap: Record<string, string> = {
      draft: '草稿',
      submitted: '已提交',
      approved: '已审批',
      rejected: '已拒绝',
      completed: '已完成',
      cancelled: '已取消',
    };
    return statusMap[status] || status;
  };

  const getStatusColor = (status: string) => {
    const colorMap: Record<string, string> = {
      draft: '#8c8c8c',
      submitted: '#1890ff',
      approved: '#52c41a',
      rejected: '#ff4d4f',
      completed: '#52c41a',
      cancelled: '#ff4d4f',
    };
    return colorMap[status] || '#8c8c8c';
  };

  const columns = [
    {
      title: '订单号',
      dataIndex: 'order_no',
      key: 'order_no',
    },
    {
      title: '供应商',
      dataIndex: 'supplier_id',
      key: 'supplier_id',
      render: (supplierId: string) => {
        const supplier = suppliers.find(s => s.id === supplierId);
        return supplier?.supplier_name || supplierId;
      }
    },
    {
      title: '订单日期',
      dataIndex: 'order_date',
      key: 'order_date',
    },
    {
      title: '预计交付日期',
      dataIndex: 'expected_delivery_date',
      key: 'expected_delivery_date',
    },
    {
      title: '总金额',
      dataIndex: 'total_amount',
      key: 'total_amount',
      render: (amount: string) => `¥${amount}`,
    },
    {
      title: '币种',
      dataIndex: 'currency',
      key: 'currency',
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => (
        <span style={{ color: getStatusColor(status) }}>
          {getStatusText(status)}
        </span>
      ),
    },
    {
      title: '操作',
      key: 'action',
      render: (_: any, record: PurchaseOrder) => (
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
          新增采购订单
        </Button>
      </div>
      <Table
        columns={columns}
        dataSource={orders}
        rowKey="id"
        loading={loading}
      />
      <Modal
        title={editingOrder ? '编辑采购订单' : '新增采购订单'}
        open={modalVisible}
        onOk={handleSubmit}
        onCancel={() => setModalVisible(false)}
        width={700}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            name="supplier_id"
            label="供应商"
            rules={[{ required: true, message: '请选择供应商' }]}
          >
            <Select disabled={!!editingOrder}>
              {suppliers.map(s => (
                <Select.Option key={s.id} value={s.id}>
                  {s.supplier_name} ({s.supplier_code})
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item
            name="order_date"
            label="订单日期"
            rules={[{ required: true, message: '请选择订单日期' }]}
          >
            <DatePicker style={{ width: '100%' }} disabled={!!editingOrder} />
          </Form.Item>
          <Form.Item name="expected_delivery_date" label="预计交付日期">
            <DatePicker style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item name="payment_terms" label="付款条件">
            <Input />
          </Form.Item>
          <Form.Item name="delivery_address" label="交付地址">
            <Input />
          </Form.Item>
          <Form.Item name="contact_person" label="联系人">
            <Input />
          </Form.Item>
          <Form.Item name="contact_phone" label="联系电话">
            <Input />
          </Form.Item>
          <Form.Item
            name="total_amount"
            label="总金额"
            rules={[{ required: true, message: '请输入总金额' }]}
          >
            <InputNumber
              style={{ width: '100%' }}
              formatter={value => `¥ ${value}`.replace(/\B(?=(\d{3})+(?!\d))/g, ',')}
              parser={value => value!.replace(/¥\s?|(,*)/g, '') as any}
            />
          </Form.Item>
          <Form.Item name="currency" label="币种">
            <Select defaultValue="CNY">
              <Select.Option value="CNY">人民币</Select.Option>
              <Select.Option value="USD">美元</Select.Option>
              <Select.Option value="EUR">欧元</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="remarks" label="备注">
            <Input.TextArea />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}
