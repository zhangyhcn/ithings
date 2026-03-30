import React, { useState, useEffect } from 'react';
import { Table, Button, Space, Modal, Form, Input, DatePicker, Select, message, Popconfirm, Card } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { scmApi } from '@/services/api';
import { useScmOrg } from '@/hooks/useScmOrg';
import dayjs from 'dayjs';

interface Stocktaking {
  id: string;
  stocktaking_no: string;
  warehouse_id: string;
  stocktaking_date: string;
  stocktaking_type: string;
  status: string;
  remarks?: string;
}

export default function StocktakingList() {
  const [stocktakings, setStocktakings] = useState<Stocktaking[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [form] = Form.useForm();
  const { orgId, tenantId } = useScmOrg();

  useEffect(() => {
    if (orgId) loadStocktakings();
  }, [orgId]);

  const loadStocktakings = async () => {
    if (!tenantId || !orgId) return;
    setLoading(true);
    try {
      const data = await scmApi.listStocktakings(tenantId, orgId);
      setStocktakings(data || []);
    } catch (error) {
      message.error('加载库存盘点失败');
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = () => {
    form.resetFields();
    setModalVisible(true);
  };

  const handleSubmit = async (values: any) => {
    if (!tenantId || !orgId) return;
    try {
      await scmApi.createStocktaking(tenantId, orgId, {
        ...values,
        stocktaking_date: values.stocktaking_date.format('YYYY-MM-DD'),
      });
      message.success('创建成功');
      setModalVisible(false);
      loadStocktakings();
    } catch (error) {
      message.error('创建失败');
    }
  };

  const handleDelete = async (id: string) => {
    if (!tenantId || !orgId) return;
    try {
      await scmApi.deleteStocktaking(tenantId, orgId, id);
      message.success('删除成功');
      loadStocktakings();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const columns = [
    { title: '盘点单号', dataIndex: 'stocktaking_no', width: 120 },
    { title: '仓库ID', dataIndex: 'warehouse_id', width: 120 },
    { title: '盘点日期', dataIndex: 'stocktaking_date', width: 120 },
    { title: '盘点类型', dataIndex: 'stocktaking_type', width: 100 },
    { title: '状态', dataIndex: 'status', width: 80 },
    { title: '备注', dataIndex: 'remarks', width: 200 },
    {
      title: '操作',
      key: 'action',
      width: 150,
      render: (_: any, record: Stocktaking) => (
        <Space>
          <Button type="link" size="small" icon={<EditOutlined />}>编辑</Button>
          <Popconfirm title="确定删除吗？" onConfirm={() => handleDelete(record.id)}>
            <Button type="link" size="small" danger icon={<DeleteOutlined />}>删除</Button>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <Card title="库存盘点管理" extra={<Button type="primary" icon={<PlusOutlined />} onClick={handleCreate}>新建盘点</Button>}>
      <Table columns={columns} dataSource={stocktakings} rowKey="id" loading={loading} />
      
      <Modal title="新建盘点单" open={modalVisible} onCancel={() => setModalVisible(false)} onOk={() => form.submit()} width={600}>
        <Form form={form} layout="vertical" onFinish={handleSubmit}>
          <Form.Item name="stocktaking_no" label="盘点单号" rules={[{ required: true }]}>
            <Input placeholder="请输入盘点单号" />
          </Form.Item>
          <Form.Item name="warehouse_id" label="仓库ID" rules={[{ required: true }]}>
            <Input placeholder="请输入仓库ID" />
          </Form.Item>
          <Form.Item name="stocktaking_date" label="盘点日期" rules={[{ required: true }]}>
            <DatePicker style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item name="stocktaking_type" label="盘点类型" initialValue="full">
            <Select options={[{ label: '全盘', value: 'full' }, { label: '抽盘', value: 'partial' }]} />
          </Form.Item>
          <Form.Item name="remarks" label="备注">
            <Input.TextArea rows={3} placeholder="请输入备注" />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}
