import React, { useState, useEffect } from 'react';
import { Table, Button, Space, Modal, Form, Input, Select, message, Popconfirm, Card } from 'antd';
import { PlusOutlined, DeleteOutlined } from '@ant-design/icons';
import { scmApi } from '@/services/api';
import { useScmOrg } from '@/hooks/useScmOrg';

interface OutboundOrder {
  id: string;
  order_no: string;
  order_type: string;
  warehouse_id: string;
  status: string;
  total_qty: string;
  remark?: string;
  created_at: string;
}

export default function OutboundOrderList() {
  const [orders, setOrders] = useState<OutboundOrder[]>([]);
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
      const data = await scmApi.listOutboundOrders(tenantId, orgId);
      setOrders(data);
    } catch (error) {
      message.error('加载出库单列表失败');
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
      await scmApi.deleteOutboundOrder(tenantId, orgId, id);
      message.success('删除成功');
      loadOrders();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      await scmApi.createOutboundOrder(tenantId, orgId, values);
      message.success('创建成功');
      setModalVisible(false);
      loadOrders();
    } catch (error) {
      message.error('创建失败');
    }
  };

  const columns = [
    { title: '出库单号', dataIndex: 'order_no', key: 'order_no' },
    { title: '出库类型', dataIndex: 'order_type', key: 'order_type' },
    { title: '仓库ID', dataIndex: 'warehouse_id', key: 'warehouse_id' },
    { title: '状态', dataIndex: 'status', key: 'status' },
    { title: '总数量', dataIndex: 'total_qty', key: 'total_qty' },
    { title: '备注', dataIndex: 'remark', key: 'remark' },
    { title: '创建时间', dataIndex: 'created_at', key: 'created_at' },
    {
      title: '操作',
      key: 'action',
      render: (_: any, record: OutboundOrder) => (
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
          新增出库单
        </Button>
      </Space>
      <Table columns={columns} dataSource={orders} rowKey="id" loading={loading} />
      <Modal
        title="新增出库单"
        open={modalVisible}
        onOk={handleSubmit}
        onCancel={() => setModalVisible(false)}
      >
        <Form form={form} layout="vertical">
          <Form.Item name="order_type" label="出库类型" rules={[{ required: true }]}>
            <Select>
              <Select.Option value="sales">销售出库</Select.Option>
              <Select.Option value="production">生产出库</Select.Option>
              <Select.Option value="transfer">调拨出库</Select.Option>
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
