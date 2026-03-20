import { useState, useEffect } from 'react';
import { ProTable, ProColumns } from '@ant-design/pro-components';
import { Button, Modal, Form, Input, message, Space, Popconfirm, Select } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { siteApi, organizationApi } from '@/services/api';
import type { Site, Organization } from '@/types';

export default function SiteList() {
  const [tableData, setTableData] = useState<Site[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<Site | null>(null);
  const [selectedTenantId, setSelectedTenantId] = useState<string>('');
  const [form] = Form.useForm();
  const [organizations, setOrganizations] = useState<Organization[]>([]);

  const flattenOrganizations = (orgs: Organization[]): Organization[] => {
    return orgs.reduce<Organization[]>((acc, org) => {
      acc.push(org);
      if (org.children && org.children.length > 0) {
        acc.push(...flattenOrganizations(org.children));
      }
      return acc;
    }, []);
  };

  const fetchData = async () => {
    const userStr = localStorage.getItem('user');
    if (!userStr) {
      setTableData([]);
      setLoading(false);
      return;
    }
    const user = JSON.parse(userStr);
    const tenantId = user.tenant_id;
    if (!tenantId) {
      console.error('No tenant_id found');
      setTableData([]);
      setLoading(false);
      return;
    }
    setSelectedTenantId(tenantId);
    setLoading(true);
    try {
      const [siteData, orgData] = await Promise.all([
        siteApi.list(tenantId, { page: 1, page_size: 100 }),
        organizationApi.list(tenantId),
      ]);
      console.log('siteData:', siteData);
      console.log('orgData:', orgData);
      setTableData(Array.isArray(siteData) ? siteData : []);
      setOrganizations(flattenOrganizations(Array.isArray(orgData) ? orgData : []));
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
      const userStr = localStorage.getItem('user');
      const user = JSON.parse(userStr || '{}');
      const tenantId = user.tenant_id || selectedTenantId;
      
      if (editingRecord) {
        await siteApi.update(tenantId, editingRecord.id, values);
        message.success('更新成功');
      } else {
        await siteApi.create(tenantId, values);
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

  const handleEdit = (record: Site) => {
    setEditingRecord(record);
    form.setFieldsValue(record);
    setModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    if (!selectedTenantId) return;
    try {
      await siteApi.delete(selectedTenantId, id);
      message.success('删除成功');
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const getOrganizationName = (orgId: string) => {
    const org = organizations.find(o => o.id === orgId);
    return org ? org.name : orgId;
  };

  const columns: ProColumns<Site>[] = [
    {
      title: '所属组织',
      dataIndex: 'organization_id',
      key: 'organization_id',
      render: (orgId: string) => getOrganizationName(orgId),
    },
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
      title: '位置',
      dataIndex: 'location',
      key: 'location',
    },
    {
      title: '描述',
      dataIndex: 'description',
      key: 'description',
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
        headerTitle="站点列表"
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
              setEditingRecord(null);
              form.resetFields();
              setModalVisible(true);
            }}
          >
            新建
          </Button>,
        ]}
      />

      <Modal
        title={editingRecord ? '编辑站点' : '新建站点'}
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
            name="organization_id"
            label="所属组织"
            rules={[{ required: true, message: '请选择组织' }]}
          >
            <Select
              placeholder="请选择组织"
              showSearch
              optionFilterProp="children"
            >
              {organizations.map((org) => (
                <Select.Option key={org.id} value={org.id}>
                  {org.name}
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
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
            disabled={!!editingRecord}
          >
            <Input placeholder="请输入标识 (如: factory-a)" />
          </Form.Item>
          <Form.Item name="location" label="位置">
            <Input placeholder="请输入位置" />
          </Form.Item>
          <Form.Item name="description" label="描述">
            <Input.TextArea placeholder="请输入描述" />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
}
