import { useState, useEffect } from 'react';
import { ProTable, ProColumns } from '@ant-design/pro-components';
import { Button, Modal, Form, Input, message, Space, Popconfirm, Select } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { deviceGroupApi, organizationApi, siteApi, driverApi } from '@/services/api';
import type { DeviceGroup, Organization, Site, Driver } from '@/types';

export default function DeviceGroupList() {
  const [tableData, setTableData] = useState<DeviceGroup[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<DeviceGroup | null>(null);
  const [tenantId, setTenantId] = useState<string>('');
  const [form] = Form.useForm();
  const [organizations, setOrganizations] = useState<Organization[]>([]);
  const [sites, setSites] = useState<Site[]>([]);
  const [drivers, setDrivers] = useState<Driver[]>([]);
  const [selectedOrgId, setSelectedOrgId] = useState<string>('');

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
      const [orgData, driverData, groupData] = await Promise.all([
        organizationApi.list(user.tenant_id),
        driverApi.list(user.tenant_id),
        deviceGroupApi.list(user.tenant_id),
      ]);
      setOrganizations(Array.isArray(orgData) ? orgData : []);
      setDrivers(Array.isArray(driverData) ? driverData : []);
      setTableData(Array.isArray(groupData) ? groupData : []);
    } catch (error) {
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  const fetchSites = async (orgId: string) => {
    if (!tenantId || !orgId) return;
    try {
      const siteData = await siteApi.list(tenantId);
      const orgSites = Array.isArray(siteData.list) 
        ? siteData.list.filter((s: Site) => s.organization_id === orgId)
        : [];
      setSites(orgSites);
    } catch (error) {
      console.error(error);
    }
  };

  const handleAdd = async (values: any) => {
    if (!tenantId) return;
    try {
      if (editingRecord) {
        await deviceGroupApi.update(tenantId, editingRecord.id, values);
        message.success('更新成功');
      } else {
        await deviceGroupApi.create(tenantId, values);
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

  const handleEdit = (record: DeviceGroup) => {
    setEditingRecord(record);
    form.setFieldsValue(record);
    setSelectedOrgId(record.org_id);
    fetchSites(record.org_id);
    setModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    if (!tenantId) return;
    try {
      await deviceGroupApi.delete(tenantId, id);
      message.success('删除成功');
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const columns: ProColumns<DeviceGroup>[] = [
    {
      title: '设备组名称',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: '所属组织',
      dataIndex: 'org_id',
      key: 'org_id',
      render: (orgId: string) => {
        const org = organizations.find(o => o.id === orgId);
        return org ? org.name : orgId;
      },
    },
    {
      title: '所属站点',
      dataIndex: 'site_id',
      key: 'site_id',
      render: (siteId: string) => {
        const site = sites.find(s => s.id === siteId);
        return site ? site.name : siteId;
      },
    },
    {
      title: '驱动镜像',
      dataIndex: 'driver_image',
      key: 'driver_image',
      ellipsis: true,
    },
    {
      title: '描述',
      dataIndex: 'description',
      key: 'description',
      ellipsis: true,
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
        headerTitle="设备组列表"
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
            创建设备组
          </Button>,
        ]}
      />

      <Modal
        title={editingRecord ? '编辑设备组' : '创建设备组'}
        open={modalVisible}
        onCancel={() => {
          setModalVisible(false);
          form.resetFields();
          setEditingRecord(null);
          setSelectedOrgId('');
          setSites([]);
        }}
        onOk={form.submit}
        width={600}
      >
        <Form form={form} layout="vertical" onFinish={handleAdd}>
          <Form.Item
            name="org_id"
            label="所属组织"
            rules={[{ required: true, message: '请选择组织' }]}
          >
            <Select
              placeholder="请选择组织"
              showSearch
              optionFilterProp="children"
              onChange={(value) => {
                setSelectedOrgId(value);
                form.setFieldsValue({ site_id: undefined });
                fetchSites(value);
              }}
            >
              {organizations.map((org) => (
                <Select.Option key={org.id} value={org.id}>
                  {org.name}
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item
            name="site_id"
            label="所属站点"
            rules={[{ required: true, message: '请选择站点' }]}
          >
            <Select
              placeholder="请选择站点"
              showSearch
              optionFilterProp="children"
              disabled={!selectedOrgId}
            >
              {sites.map((site) => (
                <Select.Option key={site.id} value={site.id}>
                  {site.name}
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item
            name="name"
            label="设备组名称"
            rules={[{ required: true, message: '请输入设备组名称' }]}
          >
            <Input placeholder="请输入设备组名称" />
          </Form.Item>
          <Form.Item
            name="driver_image"
            label="驱动镜像"
            rules={[{ required: true, message: '请选择驱动镜像' }]}
          >
            <Select
              placeholder="请选择驱动镜像"
              showSearch
              optionFilterProp="children"
            >
              {drivers.map((driver) => (
                <Select.Option key={driver.id} value={driver.image}>
                  {driver.name} ({driver.image})
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item
            name="description"
            label="描述"
          >
            <Input.TextArea placeholder="请输入描述" rows={3} />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
}
