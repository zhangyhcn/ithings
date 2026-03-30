import { useState, useEffect } from 'react';
import { ProTable, ProColumns } from '@ant-design/pro-components';
import { Button, Modal, Form, Input, message, Space, Popconfirm, Switch, InputNumber, Select } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { organizationApi } from '@/services/api';
import type { Organization } from '@/types';

export default function OrganizationList() {
  const [tableData, setTableData] = useState<Organization[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<Organization | null>(null);
  const [form] = Form.useForm();
  const [selectedTenantId, setSelectedTenantId] = useState<string>('');

  const fetchData = async () => {
    const userStr = localStorage.getItem('user');
    if (!userStr) {
      setTableData([]);
      setLoading(false);
      return;
    }
    const user = JSON.parse(userStr);
    setSelectedTenantId(user.tenant_id);
    setLoading(true);
    try {
      const data = await organizationApi.list(user.tenant_id);
      setTableData(Array.isArray(data) ? data : []);
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
        await organizationApi.update(tenantId, editingRecord.id, values);
        message.success('更新成功');
      } else {
        await organizationApi.create(tenantId, values);
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

  const handleEdit = (record: Organization) => {
    setEditingRecord(record);
    form.setFieldsValue(record);
    setModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    try {
      await organizationApi.delete(selectedTenantId, id);
      message.success('删除成功');
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const handleToggleStatus = async (id: string, checked: boolean) => {
    try {
      await organizationApi.update(selectedTenantId, id, { status: checked ? 'active' : 'inactive' });
      message.success('状态更新成功');
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const columns: ProColumns<Organization>[] = [
    {
      title: '名称',
      dataIndex: 'name',
      key: 'name',
      width: 200,
    },
    {
      title: '上级机构',
      dataIndex: 'parent_id',
      key: 'parent_id',
      width: 150,
      render: (parent_id: string) => {
        if (!parent_id) return '顶级机构';
        const findOrg = (list: Organization[]): Organization | undefined => {
          for (const org of list) {
            if (org.id === parent_id) return org;
            if (org.children) {
              const found = findOrg(org.children);
              if (found) return found;
            }
          }
          return undefined;
        };
        const org = findOrg(tableData);
        return org ? org.name : parent_id;
      },
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
            description="删除后子机构也会被删除，是否继续?"
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

  const buildTreeOptions = (list: Organization[], level = 0): any[] => {
    return list.map(item => {
      const prefix = level > 0 ? '├' + '─'.repeat(level * 2) + ' ' : '';
      const options = [{
        value: item.id,
        label: prefix + item.name,
      }];
      if (item.children && item.children.length > 0) {
        options.push(...buildTreeOptions(item.children, level + 1));
      }
      return options;
    }).flat();
  };

  const parentOptions = [
    { value: '', label: '顶级机构' },
    ...buildTreeOptions(tableData)
  ];

  return (
    <div>
      <ProTable
        headerTitle="组织机构管理"
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
            disabled={!selectedTenantId}
          >
            创建组织机构
          </Button>,
        ]}
        expandable={{ defaultExpandAllRows: true }}
      />

      <Modal
        title={editingRecord ? '编辑组织机构' : '创建组织机构'}
        open={modalVisible}
        onCancel={() => {
          setModalVisible(false);
          form.resetFields();
          setEditingRecord(null);
        }}
        onOk={form.submit}
        width={500}
      >
        <Form form={form} layout="vertical" onFinish={handleAdd}>
          <Form.Item
            name="name"
            label="名称"
            rules={[{ required: true, message: '请输入名称' }]}
          >
            <Input placeholder="请输入组织机构名称" />
          </Form.Item>
          <Form.Item
            name="parent_id"
            label="上级机构"
          >
            <Select placeholder="请选择上级机构">
              {parentOptions.map(option => (
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
            initialValue="active"
            getValueFromEvent={(checked: boolean) => checked ? 'active' : 'inactive'}
            getValueProps={(value) => ({ checked: value === 'active' })}
          >
            <Switch checkedChildren="启用" unCheckedChildren="禁用" />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
}
