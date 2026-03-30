import React, { useState, useEffect } from 'react';
import { Table, Button, Space, Modal, Form, Input, Select, message, Popconfirm, Card, InputNumber } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { scmApi } from '@/services/api';
import { useScmOrg } from '@/hooks/useScmOrg';

interface InboundOrder {
  id: string;
  order_no: string;
  order_type: string;
  warehouse_id: string;
  status: string;
  total_qty: string;
  remark?: string;
  created_at: string;
}

export default function InboundOrderList() {
  const [orders, setOrders] = useState<InboundOrder[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [form] = Form.useForm();

  const { orgId, tenantId, loading: orgLoading, error: orgError } = useScmOrg();

  useEffect(() => {
    if (orgId) {
      loadOrders();
    }
  }, [orgId]);

  const loadOrders = async () => {
    setLoading(true);
    try {
      const data = await scmApi.listInboundOrders(tenantId, orgId);
      setOrders(data);
    } catch (error) {
      message.error('加载入库单列表失败');
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = () => {
    form.resetFields();
    setModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    try {
      await scmApi.deleteInboundOrder(tenantId, orgId, id);
      message.success('删除成功');
      loadOrders();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      await scmApi.createInboundOrder(tenantId, orgId, values);
      message.success('创建成功');
      setModalVisible(false);
      loadOrders();
    } catch (error) {
      message.error('创建失败');
    }
  };

  const columns = [
    { title: '入库单号', dataIndex: 'order_no', key: 'order_no' },
    { title: '入库类型', dataIndex: 'order_type', key: 'order_type' },
    { title: '仓库ID', dataIndex: 'warehouse_id', key: 'warehouse_id' },
    { title: '状态', dataIndex: 'status', key: 'status' },
    { title: '总数量', dataIndex: 'total_qty', key: 'total_qty' },
    { title: '备注', dataIndex: 'remark', key: 'remark' },
    { title: '创建时间', dataIndex: 'created_at', key: 'created_at' },
    {
      title: '操作',
      key: 'action',
      render: (_: any, record: InboundOrder) => (
        <Space>
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
      <Space style={{ marginBottom: 16 }}>
        <Button type="primary" icon={<PlusOutlined />} onClick={handleCreate}>
          新增入库单
        </Button>
      </Space>
      <Table columns={columns} dataSource={orders} rowKey="id" loading={loading} />
      <Modal
        title="新增入库单"
        open={modalVisible}
        onOk={handleSubmit}
        onCancel={() => setModalVisible(false)}
      >
        <Form form={form} layout="vertical">
          <Form.Item name="order_type" label="入库类型" rules={[{ required: true }]}>
            <Select>
              <Select.Option value="purchase">采购入库</Select.Option>
              <Select.Option value="return">退货入库</Select.Option>
              <Select.Option value="transfer">调拨入库</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="warehouse_id" label="仓库ID" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <Input.TextArea />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}
