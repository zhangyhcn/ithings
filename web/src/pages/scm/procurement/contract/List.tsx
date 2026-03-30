import React, { useState, useEffect } from 'react';
import { Table, Button, Space, Modal, Form, Input, DatePicker, InputNumber, message, Popconfirm, Card, Select } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { scmApi } from '@/services/api';
import { useScmOrg } from '@/hooks/useScmOrg';
import dayjs from 'dayjs';

interface Contract {
  id: string;
  contract_no: string;
  supplier_id: string;
  title: string;
  start_date: string;
  end_date: string;
  total_amount: string;
  currency: string;
  payment_terms?: string;
  status: string;
}

export default function ContractList() {
  const [contracts, setContracts] = useState<Contract[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [form] = Form.useForm();
  const { orgId, tenantId } = useScmOrg();

  useEffect(() => {
    if (orgId) loadContracts();
  }, [orgId]);

  const loadContracts = async () => {
    if (!tenantId || !orgId) return;
    setLoading(true);
    try {
      const data = await scmApi.listContracts(tenantId, orgId);
      setContracts(data || []);
    } catch (error) {
      message.error('加载采购合同失败');
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
      await scmApi.createContract(tenantId, orgId, {
        ...values,
        start_date: values.start_date.format('YYYY-MM-DD'),
        end_date: values.end_date.format('YYYY-MM-DD'),
      });
      message.success('创建成功');
      setModalVisible(false);
      loadContracts();
    } catch (error) {
      message.error('创建失败');
    }
  };

  const handleDelete = async (id: string) => {
    if (!tenantId || !orgId) return;
    try {
      await scmApi.deleteContract(tenantId, orgId, id);
      message.success('删除成功');
      loadContracts();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const columns = [
    { title: '合同编号', dataIndex: 'contract_no', width: 120 },
    { title: '合同标题', dataIndex: 'title', width: 200 },
    { title: '供应商ID', dataIndex: 'supplier_id', width: 120 },
    { title: '开始日期', dataIndex: 'start_date', width: 120 },
    { title: '结束日期', dataIndex: 'end_date', width: 120 },
    { title: '总金额', dataIndex: 'total_amount', width: 120 },
    { title: '币种', dataIndex: 'currency', width: 80 },
    { title: '状态', dataIndex: 'status', width: 80 },
    {
      title: '操作',
      key: 'action',
      width: 150,
      render: (_: any, record: Contract) => (
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
    <Card title="采购合同管理" extra={<Button type="primary" icon={<PlusOutlined />} onClick={handleCreate}>新建合同</Button>}>
      <Table columns={columns} dataSource={contracts} rowKey="id" loading={loading} />
      
      <Modal title="新建合同" open={modalVisible} onCancel={() => setModalVisible(false)} onOk={() => form.submit()} width={600}>
        <Form form={form} layout="vertical" onFinish={handleSubmit}>
          <Form.Item name="contract_no" label="合同编号" rules={[{ required: true }]}>
            <Input placeholder="请输入合同编号" />
          </Form.Item>
          <Form.Item name="title" label="合同标题" rules={[{ required: true }]}>
            <Input placeholder="请输入合同标题" />
          </Form.Item>
          <Form.Item name="supplier_id" label="供应商ID" rules={[{ required: true }]}>
            <Input placeholder="请输入供应商ID" />
          </Form.Item>
          <Form.Item name="start_date" label="开始日期" rules={[{ required: true }]}>
            <DatePicker style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item name="end_date" label="结束日期" rules={[{ required: true }]}>
            <DatePicker style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item name="total_amount" label="总金额" rules={[{ required: true }]}>
            <InputNumber style={{ width: '100%' }} placeholder="请输入总金额" />
          </Form.Item>
          <Form.Item name="currency" label="币种" initialValue="CNY">
            <Select options={[{ label: 'CNY', value: 'CNY' }, { label: 'USD', value: 'USD' }]} />
          </Form.Item>
          <Form.Item name="payment_terms" label="付款条款">
            <Input.TextArea rows={2} placeholder="请输入付款条款" />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}
