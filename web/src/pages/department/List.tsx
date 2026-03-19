import { useState, useEffect } from 'react';
import { ProTable, ProColumns } from '@ant-design/pro-components';
import { Button, Modal, Form, Input, message, Space, Popconfirm, Switch, InputNumber, Select } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, ApartmentOutlined } from '@ant-design/icons';
import { departmentApi, organizationApi } from '@/services/api';
import type { Department, Organization } from '@/types';

const { Option } = Select;

export default function DepartmentList() {
  const [tableData, setTableData] = useState<Department[]>([]);
  const [organizations, setOrganizations] = useState<Organization[]>([]);
  const [selectedTenantId, setSelectedTenantId] = useState<string>('');
  const [selectedOrgId, setSelectedOrgId] = useState<string>('');
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<Department | null>(null);
  const [form] = Form.useForm();

  const fetchOrganizations = async () => {
    const userStr = localStorage.getItem('user');
    if (!userStr) {
      setOrganizations([]);
      setTableData([]);
      setLoading(false);
      return;
    }
    const user = JSON.parse(userStr);
    setSelectedTenantId(user.tenant_id);
    try {
      const data = await organizationApi.list(user.tenant_id);
      const orgList = Array.isArray(data) ? data : [];
      setOrganizations(orgList);
      if (orgList.length > 0 && !selectedOrgId) {
        setSelectedOrgId(orgList[0].id);
      } else if (orgList.length === 0) {
        setSelectedOrgId('');
        setTableData([]);
      }
    } catch (error) {
      console.error(error);
    }
  };

  const fetchData = async () => {
    if (!selectedTenantId || !selectedOrgId) {
      setTableData([]);
      return;
    }
    setLoading(true);
    try {
      const data = await departmentApi.list(selectedTenantId, selectedOrgId);
      setTableData(Array.isArray(data) ? data : []);
    } catch (error) {
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchOrganizations();
  }, []);

  useEffect(() => {
    if (selectedOrgId) {
      fetchData();
    }
  }, [selectedOrgId]);

  const handleAdd = async (values: any) => {
    try {
      const userStr = localStorage.getItem('user');
      const user = JSON.parse(userStr || '{}');
      const tenantId = user.tenant_id || selectedTenantId;
      
      if (editingRecord) {
        await departmentApi.update(tenantId, selectedOrgId, editingRecord.id, values);
        message.success('更新成功');
      } else {
        await departmentApi.create(tenantId, selectedOrgId, values);
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

  const handleEdit = (record: Department) => {
    setEditingRecord(record);
    form.setFieldsValue(record);
    setModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    try {
      await departmentApi.delete(selectedTenantId, selectedOrgId, id);
      message.success('删除成功');
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const handleToggleStatus = async (id: string, checked: boolean) => {
    try {
      await departmentApi.update(selectedTenantId, selectedOrgId, id, { status: checked ? 'active' : 'inactive' });
      message.success('状态更新成功');
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const getOrganizationName = (orgId: string) => {
    const findOrg = (list: Organization[]): Organization | undefined => {
      for (const org of list) {
        if (org.id === orgId) return org;
        if (org.children) {
          const found = findOrg(org.children);
          if (found) return found;
        }
      }
      return undefined;
    };
    const org = findOrg(organizations);
    return org ? org.name : orgId;
  };

  const getParentDepartmentName = (parentId: string) => {
    if (!parentId) return '顶级部门';
    const findDept = (list: Department[]): Department | undefined => {
      for (const dept of list) {
        if (dept.id === parentId) return dept;
        if (dept.children) {
          const found = findDept(dept.children);
          if (found) return found;
        }
      }
      return undefined;
    };
    const dept = findDept(tableData);
    return dept ? dept.name : parentId;
  };

  const buildOrgOptions = (list: Organization[], level = 0): any[] => {
    return list.map(item => {
      const prefix = level > 0 ? '├' + '─'.repeat(level * 2) + ' ' : '';
      const options = [{
        value: item.id,
        label: prefix + item.name,
      }];
      if (item.children && item.children.length > 0) {
        options.push(...buildOrgOptions(item.children, level + 1));
      }
      return options;
    }).flat();
  };

  const buildDeptOptions = (list: Department[], level = 0): any[] => {
    return list.map(item => {
      const prefix = level > 0 ? '├' + '─'.repeat(level * 2) + ' ' : '';
      const options = [{
        value: item.id,
        label: prefix + item.name,
      }];
      if (item.children && item.children.length > 0) {
        options.push(...buildDeptOptions(item.children, level + 1));
      }
      return options;
    }).flat();
  };

  const columns: ProColumns<Department>[] = [
    {
      title: '名称',
      dataIndex: 'name',
      key: 'name',
      width: 200,
    },
    {
      title: '所属机构',
      dataIndex: 'organization_id',
      key: 'organization_id',
      width: 150,
      render: (orgId: string) => getOrganizationName(orgId),
    },
    {
      title: '上级部门',
      dataIndex: 'parent_id',
      key: 'parent_id',
      width: 150,
      render: (parentId: string) => getParentDepartmentName(parentId),
    },
    {
      title: '描述',
      dataIndex: 'description',
      key: 'description',
      ellipsis: true,
    },
    {
      title: '排序',
      dataIndex: 'sort_order',
      key: 'sort_order',
      width: 80,
      align: 'center',
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      width: 100,
      align: 'center',
      render: (status: string, record) => (
        <Switch 
          checked={status === 'active'} 
          onChange={(checked) => handleToggleStatus(record.id, checked)}
          checkedChildren="启用"
          unCheckedChildren="禁用"
        />
      ),
    },
    {
      title: '创建时间',
      dataIndex: 'created_at',
      key: 'created_at',
      width: 180,
      render: (text: string) => new Date(text).toLocaleString(),
    },
    {
      title: '操作',
      key: 'action',
      width: 180,
      align: 'center',
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
            description="删除后子部门也会被删除，是否继续?"
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

  const orgOptions = buildOrgOptions(organizations);
  const parentDeptOptions = [
    { value: '', label: '顶级部门' },
    ...buildDeptOptions(tableData)
  ];

  return (
    <div>
      <div style={{ marginBottom: 16, display: 'flex', gap: 16, alignItems: 'center' }}>
        <Select
          placeholder="选择组织机构"
          style={{ width: 250 }}
          value={selectedOrgId}
          onChange={setSelectedOrgId}
          disabled={!selectedTenantId}
          options={buildOrgOptions(organizations)}
        />
      </div>
      <ProTable
        headerTitle="部门管理"
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
            disabled={!selectedTenantId || !selectedOrgId}
          >
            创建部门
          </Button>,
        ]}
        expandable={{ defaultExpandAllRows: true }}
      />

      <Modal
        title={editingRecord ? '编辑部门' : '创建部门'}
        open={modalVisible}
        onCancel={() => {
          setModalVisible(false);
          form.resetFields();
          setEditingRecord(null);
        }}
        onOk={form.submit}
        width={550}
      >
        <Form form={form} layout="vertical" onFinish={handleAdd}>
          <Form.Item
            name="organization_id"
            label="所属组织机构"
            rules={[{ required: true, message: '请选择所属组织机构' }]}
            initialValue={selectedOrgId}
          >
            <Select placeholder="请选择所属组织机构" disabled>
              {orgOptions.map(option => (
                <Option key={option.value} value={option.value}>
                  {option.label}
                </Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item
            name="name"
            label="名称"
            rules={[{ required: true, message: '请输入部门名称' }]}
          >
            <Input placeholder="请输入部门名称" />
          </Form.Item>
          <Form.Item
            name="parent_id"
            label="上级部门"
          >
            <Select placeholder="请选择上级部门">
              {parentDeptOptions.map(option => (
                <Option key={option.value} value={option.value} disabled={editingRecord && option.value === editingRecord.id}>
                  {option.label}
                </Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item
            name="description"
            label="描述"
          >
            <Input.TextArea placeholder="请输入描述" rows={3} />
          </Form.Item>
          <Form.Item
            name="sort_order"
            label="排序"
            initialValue={0}
          >
            <InputNumber min={0} style={{ width: '100%' }} placeholder="排序数值，越小越靠前" />
          </Form.Item>
          <Form.Item
            name="status"
            label="状态"
            valuePropName="checked"
            initialValue={true}
            getValueFromEvent={(e) => e.target.checked ? 'active' : 'inactive'}
            getValueProps={(value) => ({ checked: value === 'active' })}
          >
            <Switch checkedChildren="启用" unCheckedChildren="禁用" />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
}
