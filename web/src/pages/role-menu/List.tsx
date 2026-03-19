import { useState, useEffect } from 'react';
import { ProTable, ProColumns } from '@ant-design/pro-components';
import { Button, Modal, Form, Tree, Input, message, Space } from 'antd';
import { EditOutlined } from '@ant-design/icons';
import { roleApi, menuApi, roleMenuApi } from '@/services/api';
import type { Role, Menu } from '@/types';

const { TreeNode } = Tree;

export default function RoleMenuList() {
  const [tableData, setTableData] = useState<Role[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<Role | null>(null);
  const [menuTree, setMenuTree] = useState<Menu[]>([]);
  const [checkedKeys, setCheckedKeys] = useState<string[]>([]);
  const [form] = Form.useForm();

  const fetchData = async () => {
    setLoading(true);
    try {
      const data = await roleApi.list();
      setTableData(Array.isArray(data) ? data : []);
    } catch (error) {
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  const fetchMenuTree = async () => {
    try {
      const data = await menuApi.list();
      setMenuTree(Array.isArray(data) ? data : []);
    } catch (error) {
      console.error(error);
    }
  };

  useEffect(() => {
    fetchData();
    fetchMenuTree();
  }, []);

  const handleAssign = async () => {
    if (!editingRecord) return;
    
    try {
      await roleMenuApi.assignMenus({
        role_id: editingRecord.id,
        menu_ids: checkedKeys,
      });
      message.success('菜单权限分配成功');
      setModalVisible(false);
      setEditingRecord(null);
      setCheckedKeys([]);
    } catch (error) {
      console.error(error);
    }
  };

  const handleEdit = async (record: Role) => {
    setEditingRecord(record);
    try {
      const roleMenus = await roleMenuApi.getRoleMenus(record.id);
      const menuIds = roleMenus.map((rm: any) => rm.menu_id);
      setCheckedKeys(menuIds);
      setModalVisible(true);
    } catch (error) {
      console.error(error);
    }
  };

  const renderMenuTree = (menus: Menu[]): React.ReactNode => {
    return menus.map((menu) => (
      <TreeNode
        key={menu.id}
        title={menu.name}
      >
        {menu.children && menu.children.length > 0 && renderMenuTree(menu.children)}
      </TreeNode>
    ));
  };

  const columns: ProColumns<Role>[] = [
    {
      title: '角色名称',
      dataIndex: 'name',
      key: 'name',
      width: 150,
    },
    {
      title: '角色标识',
      dataIndex: 'slug',
      key: 'slug',
      width: 150,
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
      width: 100,
      render: (_, record) => (
        <span className={record.status === 'active' ? 'text-green-600' : 'text-red-600'}>
          {record.status === 'active' ? '启用' : '禁用'}
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
            分配菜单
          </Button>
        </Space>
      ),
    },
  ];

  return (
    <div>
      <ProTable<Role>
        headerTitle="角色菜单授权"
        rowKey="id"
        loading={loading}
        dataSource={tableData}
        columns={columns}
        search={false}
        pagination={false}
      />

      <Modal
        title="分配菜单权限"
        open={modalVisible}
        onCancel={() => {
          setModalVisible(false);
          setEditingRecord(null);
          setCheckedKeys([]);
        }}
        onOk={handleAssign}
        width={600}
      >
        <Form
          form={form}
          layout="vertical"
        >
          <Form.Item
            label="角色名称"
          >
            <Input disabled value={editingRecord?.name} />
          </Form.Item>

          <Form.Item
            label="选择菜单权限"
            rules={[{ required: true, message: '请选择菜单权限' }]}
          >
            <Tree
              checkable
              checkedKeys={checkedKeys}
              onCheck={(keys) => setCheckedKeys(keys as string[])}
              defaultExpandAll
            >
              {renderMenuTree(menuTree)}
            </Tree>
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
}
