import { useState, useEffect } from 'react';
import { ProTable, ProColumns } from '@ant-design/pro-components';
import { Button, Modal, Form, Input, message, Space, Popconfirm } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { useNavigate } from 'react-router-dom';
import { tenantApi } from '@/services/api';
import type { Tenant } from '@/types';

export default function TenantList() {
  const navigate = useNavigate();
  const [tableData, setTableData] = useState<Tenant[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<Tenant | null>(null);
  const [form] = Form.useForm();

  const fetchData = async () => {
    setLoading(true);
    try {
      const data = await tenantApi.list({ page: 1, page_size: 100 });
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
    try {
      if (editingRecord) {
        const config = editingRecord.config || {};
        let remote_transport: any = config.remote_transport;
        if (values.remote_transport_config && values.remote_transport_config.trim()) {
          try {
            remote_transport = JSON.parse(values.remote_transport_config);
          } catch (e) {
            message.error('远程传输配置JSON格式错误');
            return;
          }
        }
        const payload = {
          name: values.name,
          description: values.description,
          status: editingRecord.status,
          config: {
            ...config,
            registry_url: values.registry_url,
            virtual_cluster_name: values.virtual_cluster_name,
            remote_transport,
          },
        };
        await tenantApi.update(editingRecord.id, payload);
        message.success('更新成功');
      } else {
        await tenantApi.create(values);
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

  const handleEdit = (record: Tenant) => {
    setEditingRecord(record);
    const formValues = {
      ...record,
      remote_transport_config: record.config?.remote_transport
        ? JSON.stringify(record.config.remote_transport, null, 2)
        : '',
      registry_url: record.config?.registry_url,
      virtual_cluster_name: record.config?.virtual_cluster_name,
    };
    form.setFieldsValue(formValues);
    setModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    try {
      await tenantApi.delete(id);
      message.success('删除成功');
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const columns: ProColumns<Tenant>[] = [
    {
      title: '名称',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: '标识',
      dataIndex: 'slug',
      key: 'slug',
    },
    {
      title: '描述',
      dataIndex: 'description',
      key: 'description',
    },
    {
      title: '镜像仓库',
      dataIndex: 'config',
      key: 'config',
      render: (config: any) => (
        <span>{config?.registry_url || '-'}</span>
      ),
    },
    {
      title: '虚拟集群',
      dataIndex: 'config',
      key: 'config',
      render: (config: any) => (
        <span>{config?.virtual_cluster_name || '-'}</span>
      ),
    },
    {
      title: '传输类型',
      dataIndex: 'config',
      key: 'config',
      render: (config: any) => (
        <span>{config?.remote_transport?.type || '-'}</span>
      ),
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => (
        <span style={{ color: status === 'active' ? 'green' : 'red' }}>
          {status === 'active' ? '启用' : '禁用'}
        </span>
      ),
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
        headerTitle="租户列表"
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
            onClick={() => navigate('/create_tenant')}
          >
            创建租户
          </Button>,
        ]}
      />

      <Modal
        title="编辑租户"
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
            <Input placeholder="请输入名称" />
          </Form.Item>
          <Form.Item
            name="slug"
            label="标识"
            rules={[{ required: true, message: '请输入标识' }]}
            disabled={true}
          >
            <Input placeholder="请输入标识 (如: company-a)" />
          </Form.Item>
          <Form.Item name="description" label="描述">
            <Input.TextArea placeholder="请输入描述" />
          </Form.Item>
          <Form.Item name="registry_url" label="镜像仓库地址">
            <Input placeholder="如: https://registry.example.com" />
          </Form.Item>
          <Form.Item name="virtual_cluster_name" label="虚拟集群名">
            <Input placeholder="虚拟集群名称" />
          </Form.Item>

          <div style={{ marginBottom: '16px', marginTop: '16px' }}>
            <strong>远程传输配置</strong>
          </div>

          <Form.Item name="remote_transport_config" label="远程传输配置(JSON)">
            <Input.TextArea 
              placeholder={`示例 (MQTT):
{
  "type": "mqtt",
  "broker": "tcp://localhost:1883",
  "username": "user",
  "password": "pass",
  "client_id": "client-id"
}

示例 (Kafka):
{
  "type": "kafka",
  "brokers": "localhost:9092",
  "username": "user",
  "password": "pass"
}`} 
              rows={12}
            />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
}
