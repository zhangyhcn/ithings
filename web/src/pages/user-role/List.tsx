import { useState, useEffect } from 'react';
import { ProTable, ProColumns } from '@ant-design/pro-components';
import { Button, Modal, Form, Select, Input, message, Space, Popconfirm } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { userApi, roleApi, userRoleApi } from '@/services/api';
import type { User, Role } from '@/types';

const { Option } = Select;

export default function UserRoleList() {
  const [tableData, setTableData] = useState<User[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<User | null>(null);
  const [roleList, setRoleList] = useState<Role[]>([]);
  const [form] = Form.useForm();

  const fetchData = async () => {
    setLoading(true);
    try {
      const data = await userApi.list({ page: 1, page_size: 100 });
      setTableData(Array.isArray(data.list) ? data.list : []);
    } catch (error) {
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  const fetchRoleList = async () => {
    try {
      const data = await roleApi.list();
      setRoleList(Array.isArray(data) ? data : []);
    } catch (error) {
      console.error(error);
    }
  };

  useEffect(() => {
    fetchData();
    fetchRoleList();
  }, []);

  const handleAssign = async (values: any) => {
    if (!editingRecord) return;
    
    try {
      await userRoleApi.assignRoles({
        user_id: editingRecord.id,
        role_ids: values.role_ids,
      });
      message.success('角色分配成功');
      setModalVisible(false);
      form.resetFields();
      setEditingRecord(null);
    } catch (error) {
      console.error(error);
    }
  };

  const handleEdit = async (record: User) => {
    setEditingRecord(record);
    try {
      const userRoles = await userRoleApi.getUserRoles(record.id);
      const roleIds = userRoles.map((ur: any) => ur.role_id);
      form.setFieldsValue({
        username: record.username,
        role_ids: roleIds,
      });
      setModalVisible(true);
    } catch (error) {
      console.error(error);
    }
  };

  const columns: ProColumns<User>[] = [
    {
      title: '用户名',
      dataIndex: 'username',
      key: 'username',
      width: 150,
    },
    {
      title: '邮箱',
      dataIndex: 'email',
      key: 'email',
      width: 200,
    },
    {
      title: '角色',
      dataIndex: 'role',
      key: 'role',
      width: 120,
    },
    {
      title: '状态',
      dataIndex: 'is_active',
      key: 'is_active',
      width: 100,
      render: (_, record) => (
        <span className={record.is_active ? 'text-green-600' : 'text-red-600'}>
          {record.is_active ? '启用' : '禁用'}
        </span>
      ),
    },
    {
      title: '创建时间',
      dataIndex: 'created_at',
      key: 'created_at',
      width: 180,
    },
    {
      title: '操作',
      key: 'action',
      width: 150,
      render: (_, record) => (
        <Space size="middle">
          <Button
            type="link"
            icon={<EditOutlined />}
            onClick={() => handleEdit(record)}
          >
            分配角色
          </Button>
        </Space>
      ),
    },
  ];

  return (
    <div>
      <ProTable<User>
        headerTitle="用户角色授权"
        rowKey="id"
        loading={loading}
        dataSource={tableData}
        columns={columns}
        search={false}
        pagination={false}
      />

      <Modal
        title="分配用户角色"
        open={modalVisible}
        onCancel={() => {
          setModalVisible(false);
          form.resetFields();
          setEditingRecord(null);
        }}
        footer={null}
        width={500}
      >
        <Form
          form={form}
          layout="vertical"
          onFinish={handleAssign}
        >
          <Form.Item
            name="username"
            label="用户名"
          >
            <Input disabled placeholder="用户名" />
          </Form.Item>

          <Form.Item
            name="role_ids"
            label="选择角色"
            rules={[{ required: true, message: '请选择角色' }]}
          >
            <Select
              mode="multiple"
              placeholder="请选择角色"
              optionFilterProp="children"
              showSearch
            >
              {roleList.map((role) => (
                <Option key={role.id} value={role.id}>
                  {role.name}
                </Option>
              ))}
            </Select>
          </Form.Item>

          <Form.Item>
            <Space>
              <Button type="primary" htmlType="submit">
                提交
              </Button>
              <Button
                onClick={() => {
                  setModalVisible(false);
                  form.resetFields();
                  setEditingRecord(null);
                }}
              >
                取消
              </Button>
            </Space>
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
}
