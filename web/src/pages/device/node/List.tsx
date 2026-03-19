import { useState, useEffect } from 'react';
import { ProTable, ProColumns } from '@ant-design/pro-components';
import { Button, Modal, Form, Input, Switch, message, Space, Popconfirm } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { nodeApi } from '@/services/api';
import type { Node } from '@/types';

export default function NodeList() {
  const [tableData, setTableData] = useState<Node[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<Node | null>(null);
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
      const data = await nodeApi.list(user.tenant_id);
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
        await nodeApi.update(tenantId, editingRecord.id, values);
        message.success('更新成功');
      } else {
        await nodeApi.create(tenantId, values);
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

  const handleEdit = (record: Node) => {
    setEditingRecord(record);
    form.setFieldsValue(record);
    setModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    if (!tenantId) return;
    try {
      await nodeApi.delete(tenantId, id);
      message.success('删除成功');
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const columns: ProColumns<Node>[] = [
    {
      title: '名称',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: '地址',
      dataIndex: 'address',
      key: 'address',
    },
    {
      title: '共享节点',
      dataIndex: 'is_shared',
      key: 'is_shared',
      render: (is_shared: boolean) => (
        <span style={{ color: is_shared ? 'green' : 'gray' }}>
          {is_shared ? '是' : '否'}
        </span>
      ),
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => (
        <span style={{ color: status === 'online' ? 'green' : 'red' }}>
          {status === 'online' ? '在线' : '离线'}
        </span>
      ),
    },
    {
      title: '最后同步',
      dataIndex: 'last_sync',
      key: 'last_sync',
      render: (text: string) => text ? new Date(text).toLocaleString() : '-',
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
        headerTitle="节点列表"
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
            创建节点
          </Button>,
        ]}
      />

      <Modal
        title={editingRecord ? '编辑节点' : '创建节点'}
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
            rules={[{ required: true, message: '请输入节点名称' }]}
          >
            <Input placeholder="请输入节点名称" />
          </Form.Item>
          <Form.Item
            name="address"
            label="节点地址"
          >
            <Input placeholder="请输入节点地址" />
          </Form.Item>
          <Form.Item
            name="k8s_context"
            label="K8s 上下文"
          >
            <Input placeholder="请输入 K8s 上下文名称" />
          </Form.Item>
          <Form.Item
            name="is_shared"
            label="共享节点"
            valuePropName="checked"
            initialValue={false}
          >
            <Switch checkedChildren="是" unCheckedChildren="否" />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
}
