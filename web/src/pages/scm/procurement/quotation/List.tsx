import React, { useState, useEffect } from 'react';
import { Table, Button, Space, Modal, Form, Input, DatePicker, InputNumber, message, Popconfirm, Card, Select } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { scmApi } from '@/services/api';
import { useScmOrg } from '@/hooks/useScmOrg';
import dayjs from 'dayjs';

interface Quotation {
  id: string;
  supplier_id: string;
  material_id: string;
  price: string;
  currency: string;
  min_qty: string;
  max_qty?: string;
  valid_from: string;
  valid_to: string;
  lead_time: number;
  status: string;
  created_at: string;
}

export default function QuotationList() {
  const [quotations, setQuotations] = useState<Quotation[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingQuotation, setEditingQuotation] = useState<Quotation | null>(null);
  const [form] = Form.useForm();
  const { orgId, tenantId } = useScmOrg();

  useEffect(() => {
    if (orgId) {
      loadQuotations();
    }
  }, [orgId]);

  const loadQuotations = async () => {
    if (!tenantId || !orgId) return;
    setLoading(true);
    try {
      const data = await scmApi.listQuotations(tenantId, orgId);
      setQuotations(data || []);
    } catch (error) {
      message.error('加载供应商报价失败');
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = () => {
    setEditingQuotation(null);
    form.resetFields();
    setModalVisible(true);
  };

  const handleEdit = (quotation: Quotation) => {
    setEditingQuotation(quotation);
    form.setFieldsValue({
      ...quotation,
      valid_from: dayjs(quotation.valid_from),
      valid_to: dayjs(quotation.valid_to),
    });
    setModalVisible(true);
  };

  const handleSubmit = async (values: any) => {
    if (!tenantId || !orgId) return;
    
    const data = {
      ...values,
      valid_from: values.valid_from.format('YYYY-MM-DD'),
      valid_to: values.valid_to.format('YYYY-MM-DD'),
    };

    try {
      if (editingQuotation) {
        await scmApi.updateQuotation(tenantId, orgId, editingQuotation.id, data);
        message.success('更新成功');
      } else {
        await scmApi.createQuotation(tenantId, orgId, data);
        message.success('创建成功');
      }
      setModalVisible(false);
      loadQuotations();
    } catch (error) {
      message.error('操作失败');
    }
  };

  const handleDelete = async (id: string) => {
    if (!tenantId || !orgId) return;
    try {
      await scmApi.deleteQuotation(tenantId, orgId, id);
      message.success('删除成功');
      loadQuotations();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const columns = [
    { title: '供应商ID', dataIndex: 'supplier_id', key: 'supplier_id', width: 120 },
    { title: '物料ID', dataIndex: 'material_id', key: 'material_id', width: 120 },
    { title: '单价', dataIndex: 'price', key: 'price', width: 100 },
    { title: '币种', dataIndex: 'currency', key: 'currency', width: 80 },
    { title: '最小数量', dataIndex: 'min_qty', key: 'min_qty', width: 100 },
    { title: '最大数量', dataIndex: 'max_qty', key: 'max_qty', width: 100 },
    { title: '生效日期', dataIndex: 'valid_from', key: 'valid_from', width: 120 },
    { title: '失效日期', dataIndex: 'valid_to', key: 'valid_to', width: 120 },
    { title: '交期(天)', dataIndex: 'lead_time', key: 'lead_time', width: 90 },
    { title: '状态', dataIndex: 'status', key: 'status', width: 80 },
    {
      title: '操作',
      key: 'action',
      width: 150,
      render: (_: any, record: Quotation) => (
        <Space size="small">
          <Button type="link" size="small" icon={<EditOutlined />} onClick={() => handleEdit(record)}>
            编辑
          </Button>
          <Popconfirm title="确定删除吗？" onConfirm={() => handleDelete(record.id)}>
            <Button type="link" size="small" danger icon={<DeleteOutlined />}>
              删除
            </Button>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <Card title="供应商报价管理" extra={<Button type="primary" icon={<PlusOutlined />} onClick={handleCreate}>新建报价</Button>}>
      <Table columns={columns} dataSource={quotations} rowKey="id" loading={loading} scroll={{ x: 1400 }} />
      
      <Modal
        title={editingQuotation ? '编辑报价' : '新建报价'}
        open={modalVisible}
        onCancel={() => setModalVisible(false)}
        onOk={() => form.submit()}
        width={600}
      >
        <Form form={form} layout="vertical" onFinish={handleSubmit}>
          <Form.Item name="supplier_id" label="供应商ID" rules={[{ required: true }]}>
            <Input placeholder="请输入供应商ID" />
          </Form.Item>
          <Form.Item name="material_id" label="物料ID" rules={[{ required: true }]}>
            <Input placeholder="请输入物料ID" />
          </Form.Item>
          <Form.Item name="price" label="单价" rules={[{ required: true }]}>
            <InputNumber style={{ width: '100%' }} placeholder="请输入单价" />
          </Form.Item>
          <Form.Item name="currency" label="币种" initialValue="CNY">
            <Select options={[{ label: 'CNY', value: 'CNY' }, { label: 'USD', value: 'USD' }, { label: 'EUR', value: 'EUR' }]} />
          </Form.Item>
          <Form.Item name="min_qty" label="最小数量">
            <InputNumber style={{ width: '100%' }} placeholder="请输入最小数量" />
          </Form.Item>
          <Form.Item name="max_qty" label="最大数量">
            <InputNumber style={{ width: '100%' }} placeholder="请输入最大数量" />
          </Form.Item>
          <Form.Item name="valid_from" label="生效日期" rules={[{ required: true }]}>
            <DatePicker style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item name="valid_to" label="失效日期" rules={[{ required: true }]}>
            <DatePicker style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item name="lead_time" label="交期(天)">
            <InputNumber style={{ width: '100%' }} placeholder="请输入交期" />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}
