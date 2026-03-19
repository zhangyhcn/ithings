import { useState, useEffect } from 'react';
import { ProTable, ProColumns } from '@ant-design/pro-components';
import { Button, Modal, Form, Input, Select, message, Space, Popconfirm, Tag } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, CloudUploadOutlined } from '@ant-design/icons';
import { crdApi, namespaceApi } from '@/services/api';
import type { CRD, Namespace } from '@/types';

export default function CrdList() {
  const [tableData, setTableData] = useState<CRD[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<CRD | null>(null);
  const [namespaceOptions, setNamespaceOptions] = useState<{label: string, value: string}[]>([]);
  const [form] = Form.useForm();

  const fetchData = async () => {
    setLoading(true);
    try {
      const data = await crdApi.list({ page: 1, page_size: 100 });
      setTableData(Array.isArray(data.list) ? data.list : []);
    } catch (error) {
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  const fetchNamespaces = async () => {
    try {
      const data = await namespaceApi.listByTenant('default');
      const options = data.map((ns: Namespace) => ({
        label: `${ns.name} (${ns.slug})`,
        value: ns.id,
      }));
      setNamespaceOptions(options);
    } catch (error) {
      console.error(error);
    }
  };

  useEffect(() => {
    fetchData();
    fetchNamespaces();
  }, []);

  const handleAdd = async (values: any) => {
    try {
      if (editingRecord) {
        await crdApi.update(editingRecord.id, values);
        message.success('更新成功');
      } else {
        await crdApi.create(values);
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

  const handleEdit = (record: CRD) => {
    setEditingRecord(record);
    form.setFieldsValue(record);
    setModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    try {
      await crdApi.delete(id);
      message.success('删除成功');
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const handlePublish = async (id: string) => {
    try {
      await crdApi.publish(id);
      message.success('发布成功');
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const getStatusTag = (status: string) => {
    switch (status) {
      case 'draft':
        return <Tag color="default">草稿</Tag>;
      case 'published':
        return <Tag color="success">已发布</Tag>;
      default:
        return <Tag color="default">{status}</Tag>;
    }
  };

  const columns: ProColumns<CRD>[] = [
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
      title: 'Group',
      dataIndex: 'group',
      key: 'group',
    },
    {
      title: 'Version',
      dataIndex: 'version',
      key: 'version',
    },
    {
      title: 'Kind',
      dataIndex: 'kind',
      key: 'kind',
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => getStatusTag(status),
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
          {record.status !== 'published' && (
            <Popconfirm
              title="确认发布到K8s?"
              onConfirm={() => handlePublish(record.id)}
            >
              <Button type="link" icon={<CloudUploadOutlined />}>
                发布
              </Button>
            </Popconfirm>
          )}
          {record.status === 'published' && (
            <Popconfirm
              title="确认重新发布到K8s?"
              onConfirm={() => handlePublish(record.id)}
            >
              <Button type="link" icon={<CloudUploadOutlined />}>
                重新发布
              </Button>
            </Popconfirm>
          )}
          <Popconfirm
            title="确认删除? 已发布的CRD将同时从K8s中删除"
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
        headerTitle="CRD定义列表"
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
            onClick={() => {
              setModalVisible(true);
              setEditingRecord(null);
              form.resetFields();
            }}
          >
            创建CRD
          </Button>,
        ]}
      />

      <Modal
        title={editingRecord ? '编辑CRD' : '创建CRD'}
        open={modalVisible}
        onCancel={() => {
          setModalVisible(false);
          form.resetFields();
          setEditingRecord(null);
        }}
        onOk={form.submit}
        width={600}
      >
        <Form form={form} layout="vertical" onFinish={handleAdd}>
          <Form.Item
            name="name"
            label="名称"
            rules={[{ required: true, message: '请输入名称' }]}
          >
            <Input placeholder="请输入CRD名称" />
          </Form.Item>
          <Form.Item
            name="slug"
            label="标识"
            rules={[{ required: true, message: '请输入标识' }]}
            disabled={!!editingRecord}
          >
            <Input placeholder="请输入标识，如: my-resource" />
          </Form.Item>
          <Form.Item
            name="namespace_id"
            label="命名空间"
            rules={[{ required: true, message: '请选择命名空间' }]}
          >
            <Select
            placeholder="请选择命名空间"
            options={namespaceOptions}
            showSearch
            filterOption={(input, option) =>
              (option?.label ?? '').toLowerCase().includes(input.toLowerCase())
            }
          />
          </Form.Item>
          <Form.Item
            name="group"
            label="Group"
            rules={[{ required: true, message: '请输入Group' }]}
          >
            <Input placeholder="如: example.com" />
          </Form.Item>
          <Form.Item
            name="version"
            label="Version"
            rules={[{ required: true, message: '请输入Version' }]}
          >
            <Input placeholder="如: v1" />
          </Form.Item>
          <Form.Item
            name="kind"
            label="Kind"
            rules={[{ required: true, message: '请输入Kind' }]}
          >
            <Input placeholder="如: MyResource" />
          </Form.Item>
          <Form.Item name="description" label="描述">
            <Input.TextArea placeholder="请输入描述" />
          </Form.Item>
          <Form.Item name="yaml" label="YAML配置(JSON格式)">
            <Input.TextArea placeholder="请输入完整YAML配置(JSON格式)" rows={10} />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
}
