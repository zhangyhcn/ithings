import React, { useState, useEffect } from 'react';
import { Table, Button, Space, Modal, Form, Input, DatePicker, Select, message, Popconfirm, Card } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { scmApi } from '@/services/api';
import { useScmOrg } from '@/hooks/useScmOrg';
import dayjs from 'dayjs';

interface Bidding {
  id: string;
  bidding_no: string;
  title: string;
  bidding_type: string;
  publish_date: string;
  deadline: string;
  contact_person?: string;
  contact_phone?: string;
  description?: string;
  status: string;
}

export default function BiddingList() {
  const [biddings, setBiddings] = useState<Bidding[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [form] = Form.useForm();
  const { orgId, tenantId } = useScmOrg();

  useEffect(() => {
    if (orgId) loadBiddings();
  }, [orgId]);

  const loadBiddings = async () => {
    if (!tenantId || !orgId) return;
    setLoading(true);
    try {
      const data = await scmApi.listBiddings(tenantId, orgId);
      setBiddings(data || []);
    } catch (error) {
      message.error('加载招投标失败');
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
      await scmApi.createBidding(tenantId, orgId, {
        ...values,
        publish_date: values.publish_date.format('YYYY-MM-DD'),
        deadline: values.deadline.toISOString(),
      });
      message.success('创建成功');
      setModalVisible(false);
      loadBiddings();
    } catch (error) {
      message.error('创建失败');
    }
  };

  const handleDelete = async (id: string) => {
    if (!tenantId || !orgId) return;
    try {
      await scmApi.deleteBidding(tenantId, orgId, id);
      message.success('删除成功');
      loadBiddings();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const columns = [
    { title: '招标编号', dataIndex: 'bidding_no', width: 120 },
    { title: '标题', dataIndex: 'title', width: 200 },
    { title: '招标类型', dataIndex: 'bidding_type', width: 100 },
    { title: '发布日期', dataIndex: 'publish_date', width: 120 },
    { title: '截止时间', dataIndex: 'deadline', width: 180 },
    { title: '联系人', dataIndex: 'contact_person', width: 100 },
    { title: '状态', dataIndex: 'status', width: 80 },
    {
      title: '操作',
      key: 'action',
      width: 150,
      render: (_: any, record: Bidding) => (
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
    <Card title="招投标管理" extra={<Button type="primary" icon={<PlusOutlined />} onClick={handleCreate}>新建招标</Button>}>
      <Table columns={columns} dataSource={biddings} rowKey="id" loading={loading} />
      
      <Modal title="新建招标" open={modalVisible} onCancel={() => setModalVisible(false)} onOk={() => form.submit()} width={600}>
        <Form form={form} layout="vertical" onFinish={handleSubmit}>
          <Form.Item name="bidding_no" label="招标编号" rules={[{ required: true }]}>
            <Input placeholder="请输入招标编号" />
          </Form.Item>
          <Form.Item name="title" label="标题" rules={[{ required: true }]}>
            <Input placeholder="请输入标题" />
          </Form.Item>
          <Form.Item name="bidding_type" label="招标类型" initialValue="public">
            <Select options={[{ label: '公开招标', value: 'public' }, { label: '邀请招标', value: 'invite' }]} />
          </Form.Item>
          <Form.Item name="publish_date" label="发布日期" rules={[{ required: true }]}>
            <DatePicker style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item name="deadline" label="截止时间" rules={[{ required: true }]}>
            <DatePicker showTime style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item name="contact_person" label="联系人">
            <Input placeholder="请输入联系人" />
          </Form.Item>
          <Form.Item name="contact_phone" label="联系电话">
            <Input placeholder="请输入联系电话" />
          </Form.Item>
          <Form.Item name="description" label="描述">
            <Input.TextArea rows={3} placeholder="请输入描述" />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}
