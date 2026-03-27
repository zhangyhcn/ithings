import { useState, useEffect } from 'react';
import { ProTable, ProColumns } from '@ant-design/pro-components';
import { Button, Modal, Form, Input, message, Space, Popconfirm, Select } from 'antd';
import { EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { driverApi, tenantApi } from '@/services/api';
import type { Driver, Tenant } from '@/types';

const { TextArea } = Input;
const { Option } = Select;

export default function DriverList() {
  const [tableData, setTableData] = useState<Driver[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<Driver | null>(null);
  const [tenantId, setTenantId] = useState<string>('');
  const [availableTags, setAvailableTags] = useState<string[]>([]);
  const [loadingTags, setLoadingTags] = useState(false);
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
      const drivers = Array.isArray(data) ? data : [];
      setTableData(drivers);
    } catch (error) {
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  const fetchTags = async () => {
    const imageName = form.getFieldValue('image');
    if (!imageName) {
      setAvailableTags([]);
      return;
    }
    setLoadingTags(true);
    try {
      const resp = await driverApi.listTags(tenantId, undefined, imageName);
      setAvailableTags(Array.isArray(resp) ? resp : []);
    } catch (error) {
      console.error('Failed to fetch tags:', error);
      message.error('获取镜像标签失败');
      setAvailableTags([]);
    } finally {
      setLoadingTags(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  const handleEdit = (record: Driver) => {
    setEditingRecord(record);
    form.setFieldsValue({
      ...record,
      device_profile: JSON.stringify(record.device_profile, null, 2),
    });
    setModalVisible(true);
    fetchTags();
  };

  const handleEditSubmit = async (values: any) => {
    if (!tenantId || !editingRecord) return;
    try {
      const payload = {
        ...values,
        device_profile: values.device_profile ? JSON.parse(values.device_profile) : {},
      };
      await driverApi.update(tenantId, editingRecord.id, payload);
      message.success('更新成功');
      setModalVisible(false);
      form.resetFields();
      setEditingRecord(null);
      fetchData();
    } catch (error) {
      message.error('操作失败');
      console.error(error);
    }
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
      title: '镜像名称',
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
        toolBarRender={() => []}
      />

      <Modal
        title="编辑驱动"
        open={modalVisible}
        onCancel={() => {
          setModalVisible(false);
          form.resetFields();
          setEditingRecord(null);
        }}
        onOk={form.submit}
        width={700}
      >
        <Form form={form} layout="vertical" onFinish={handleEditSubmit}>
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
            label="镜像名称"
            rules={[{ required: true, message: '请输入镜像名称' }]}
          >
            <Input 
              placeholder="如: modbus-driver" 
              onChange={() => fetchTags()}
            />
          </Form.Item>
          <Form.Item
            name="version"
            label="版本"
            rules={[{ required: true, message: '请选择版本' }]}
          >
            <Select 
              placeholder="选择版本/标签" 
              loading={loadingTags}
              allowClear
            >
              {availableTags.map(tag => (
                <Option key={tag} value={tag}>{tag}</Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item name="description" label="描述">
            <Input.TextArea placeholder="请输入描述" />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
}
