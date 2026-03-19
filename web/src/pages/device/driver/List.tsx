import { useState, useEffect } from 'react';
import { ProTable, ProColumns } from '@ant-design/pro-components';
import { Button, Modal, Form, Input, message, Space, Popconfirm } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { driverApi } from '@/services/api';
import type { Driver } from '@/types';

export default function DriverList() {
  const [tableData, setTableData] = useState<Driver[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<Driver | null>(null);
  const [tenantId, setTenantId] = useState<string>('');
  const [form] = Form.useForm();

  const fetchData = async () => {
    const userStr = localStorage.getItem('user');
    if (!userStr) {
      setTableData([]);
      setLoading(false);
      return;
    }
    const user = JSON.parse(userStr);
    setTenantId(user.tenant_id);
    setLoading(true);
    try {
      const data = await driverApi.list(user.tenant_id);
      setTableData(Array.isArray(data.list) ? data.list : []);
    } catch (error) {
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  const handleAdd = async (values: any) => {
    if (!tenantId) return;
    try {
      if (editingRecord) {
        await driverApi.update(tenantId, editingRecord.id, values);
        message.success('更新成功');
      } else {
        await driverApi.create(tenantId, values);
        message.success('创建成功');
      }
      setModalVisible(false);
      form.resetFields();
      setEditingRecord(null);
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const handleEdit = (record: Driver) => {
    setEditingRecord(record);
    form.setFieldsValue(record);
    setModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    if (!tenantId) return;
    try {
      await driverApi.delete(tenantId, id);
      message.success('删除成功');
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const columns: ProColumns<Driver>[] = [
    {
      title: '名称',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: '协议类型',
      dataIndex: 'protocol_type',
      key: 'protocol_type',
    },
    {
      title: '镜像',
      dataIndex: 'image',
      key: 'image',
    },
    {
      title: '版本',
      dataIndex: 'version',
      key: 'version',
    },
    {
      title: '描述',
      dataIndex: 'description',
      key: 'description',
    },
    {
      title: '创建时间',
      dataIndex: 'created_at',
      key: 'created_at',
      render: (text: string) => new Date(text).toLocaleString(),
    },
    {
      title: '操作',
      key: 'action',
      render: (_, record) => (
        <Space>
          <Button
            type="link"
            icon={<EditOutlined />}
            onClick={() => handleEdit(record)}
          >
            编辑
          </Button>
          <Popconfirm
            title="确认删除?"
            onConfirm={() => handleDelete(record.id)}
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
    <div>
      <ProTable
        headerTitle="驱动列表"
        columns={columns}
        dataSource={tableData}
        loading={loading}
        rowKey="id"
        search={false}
        toolBarRender={() => [
          <Button
            type="primary"
            key="add"
            icon={<PlusOutlined />}
            onClick={() => setModalVisible(true)}
          >
            创建驱动
          </Button>,
        ]}
      />

      <Modal
        title={editingRecord ? '编辑驱动' : '创建驱动'}
        open={modalVisible}
        onCancel={() => {
          setModalVisible(false);
          form.resetFields();
          setEditingRecord(null);
        }}
        onOk={form.submit}
      >
        <Form form={form} layout="vertical" onFinish={handleAdd}>
          <Form.Item
            name="name"
            label="名称"
            rules={[{ required: true, message: '请输入名称' }]}
          >
            <Input placeholder="请输入驱动名称" />
          </Form.Item>
          <Form.Item
            name="protocol_type"
            label="协议类型"
            rules={[{ required: true, message: '请输入协议类型' }]}
          >
            <Input placeholder="如: modbus-tcp, opc-ua" />
          </Form.Item>
          <Form.Item
            name="image"
            label="镜像地址"
            rules={[{ required: true, message: '请输入镜像地址' }]}
          >
            <Input placeholder="如: registry.example.com/modbus-driver:latest" />
          </Form.Item>
          <Form.Item
            name="version"
            label="版本"
            rules={[{ required: true, message: '请输入版本' }]}
          >
            <Input placeholder="如: v1.0.0" />
          </Form.Item>
          <Form.Item name="description" label="描述">
            <Input.TextArea placeholder="请输入描述" />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
}
