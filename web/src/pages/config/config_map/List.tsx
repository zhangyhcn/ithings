import { useState, useEffect } from 'react';
import { ProTable, ProColumns } from '@ant-design/pro-components';
import { Button, Modal, Form, Input, Select, message, Space, Popconfirm, Tag, Card } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, CloudUploadOutlined } from '@ant-design/icons';
import { configMapApi, namespaceApi } from '@/services/api';
import type { ConfigMap, Namespace } from '@/types';

export default function ConfigMapList() {
  const [tableData, setTableData] = useState<ConfigMap[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<ConfigMap | null>(null);
  const [namespaceOptions, setNamespaceOptions] = useState<{label: string, value: string}[]>([]);
  const [dataRows, setDataRows] = useState<string[]>([]);
  const [form] = Form.useForm();

  const fetchData = async () => {
    setLoading(true);
    try {
      const data = await configMapApi.list({ page: 1, page_size: 100 });
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
      const dataObj: Record<string, string> = {};
      dataRows.forEach((_, index) => {
        const key = values[`key${index}`];
        const value = values[`value${index}`];
        if (key && value) {
          dataObj[key] = value;
        }
      });
      values.data = dataObj;

      if (editingRecord) {
        await configMapApi.update(editingRecord.id, values);
        message.success('更新成功');
      } else {
        await configMapApi.create(values);
        message.success('创建成功');
      }
      setModalVisible(false);
      form.resetFields();
      setEditingRecord(null);
      setDataRows([]);
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const handleEdit = (record: ConfigMap) => {
    setEditingRecord(record);
    const dataObj = record.data || {};
    const entries = Object.entries(dataObj);
    const rows = entries.length > 0 ? Object.keys(dataObj) : [''];
    setDataRows(rows);
    const initialValues = { ...record };
    entries.forEach(([key, value], index) => {
      initialValues[`key${index}`] = key;
      initialValues[`value${index}`] = value;
    });
    form.setFieldsValue(initialValues);
    setModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    try {
      await configMapApi.delete(id);
      message.success('删除成功');
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const handlePublish = async (id: string) => {
    try {
      await configMapApi.publish(id);
      message.success('发布成功');
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const addDataRow = () => {
    setDataRows([...dataRows, '']);
  };

  const removeDataRow = (index: number) => {
    const newRows = [...dataRows];
    newRows.splice(index, 1);
    setDataRows(newRows);
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

  const columns: ProColumns<ConfigMap>[] = [
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
      title: '配置项数量',
      key: 'dataCount',
      render: (_, record) => {
        const data = record.data;
        if (!data) return '0';
        return Object.keys(data).length.toString();
      },
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => getStatusTag(status),
    },
    {
      title: 'K8s名称',
      dataIndex: 'k8s_name',
      key: 'k8s_name',
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
            title="确认删除? 已发布的ConfigMap将同时从K8s中删除"
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
        headerTitle="配置项列表"
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
              setDataRows(['']);
            }}
          >
            创建配置项
          </Button>,
        ]}
      />

      <Modal
        title={editingRecord ? '编辑配置项' : '创建配置项'}
        open={modalVisible}
        onCancel={() => {
          setModalVisible(false);
          form.resetFields();
          setEditingRecord(null);
          setDataRows([]);
        }}
        onOk={form.submit}
        width={700}
      >
        <Form form={form} layout="vertical" onFinish={handleAdd}>
          <Form.Item
            name="name"
            label="名称"
            rules={[{ required: true, message: '请输入名称' }]}
          >
            <Input placeholder="请输入配置项名称" />
          </Form.Item>
          <Form.Item
            name="slug"
            label="标识"
            rules={[{ required: true, message: '请输入标识' }]}
            disabled={!!editingRecord}
          >
            <Input placeholder="请输入标识，如: app-config" />
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
          <Form.Item name="description" label="描述">
            <Input.TextArea placeholder="请输入描述" />
          </Form.Item>

          <Card
            title="配置数据"
            extra={<Button type="link" onClick={addDataRow}>添加配置项</Button>}
            style={{ marginBottom: 16 }}
          >
            {dataRows.map((_, index) => (
              <div key={index} style={{ display: 'flex', gap: 8, marginBottom: 8 }}>
                <Form.Item
                  name={`key${index}`}
                  label="Key"
                  style={{ flex: 1, marginBottom: 0 }}
                >
                  <Input placeholder="配置键" />
                </Form.Item>
                <Form.Item
                  name={`value${index}`}
                  label="Value"
                  style={{ flex: 2, marginBottom: 0 }}
                >
                  <Input placeholder="配置值" />
                </Form.Item>
                {dataRows.length > 1 && (
                  <Button
                    danger
                    type="text"
                    icon={<DeleteOutlined />}
                    onClick={() => removeDataRow(index)}
                    style={{ alignSelf: 'flex-end' }}
                  />
                )}
              </div>
            ))}
          </Card>
        </Form>
      </Modal>
    </div>
  );
}
