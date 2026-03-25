import { useState, useEffect } from 'react';
import { ProTable, ProColumns } from '@ant-design/pro-components';
import { Button, Modal, Input, message, Space, Tag, Popconfirm } from 'antd';
import { EditOutlined, PlusOutlined, DeleteOutlined } from '@ant-design/icons';
import { nodeApi } from '@/services/api';
import type { Node } from '@/types';

export default function NodeList() {
  const [tableData, setTableData] = useState<Node[]>([]);
  const [loading, setLoading] = useState(false);
  const [labelModalVisible, setLabelModalVisible] = useState(false);
  const [editingNode, setEditingNode] = useState<Node | null>(null);
  const [labels, setLabels] = useState<{ key: string; value: string }[]>([]);
  const [tenantId, setTenantId] = useState<string>('');

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
      console.log('Node list response (axios returns):', data);
      console.log('data is array:', Array.isArray(data));
      console.log('data.data:', data.data);
      setTableData(Array.isArray(data) ? data : []);
    } catch (error) {
      console.error('Fetch nodes error:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleSync = async () => {
    if (!tenantId) return;
    setLoading(true);
    try {
      const data = await nodeApi.sync(tenantId);
      setTableData(Array.isArray(data.data) ? data.data : []);
      message.success('节点同步成功');
    } catch (error) {
      message.error('节点同步失败');
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  const handleManageLabels = (record: Node) => {
    setEditingNode(record);
    const labelList = record.labels
      ? Object.entries(record.labels).map(([key, value]) => ({ key, value }))
      : [];
    setLabels(labelList);
    setLabelModalVisible(true);
  };

  const handleAddLabel = () => {
    setLabels([...labels, { key: '', value: '' }]);
  };

  const handleRemoveLabel = (index: number) => {
    const newLabels = labels.filter((_, i) => i !== index);
    setLabels(newLabels);
  };

  const handleLabelChange = (index: number, field: 'key' | 'value', value: string) => {
    const newLabels = [...labels];
    newLabels[index][field] = value;
    setLabels(newLabels);
  };

  const handleSaveLabels = async () => {
    if (!editingNode || !tenantId) return;
    const labelObj: Record<string, string> = {};
    labels.forEach(label => {
      if (label.key.trim()) {
        labelObj[label.key.trim()] = label.value.trim();
      }
    });
    try {
      await nodeApi.updateLabels(tenantId, editingNode.id, { labels: labelObj });
      message.success('标签更新成功');
      setLabelModalVisible(false);
      setEditingNode(null);
      fetchData();
    } catch (error) {
      message.error('标签更新失败');
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
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => (
        <Tag color={status === 'Ready' ? 'green' : 'red'}>
          {status === 'Ready' ? '就绪' : status}
        </Tag>
      ),
    },
    {
      title: '角色',
      dataIndex: 'roles',
      key: 'roles',
      render: (roles: string[]) => (
        <Space>
          {roles?.map(role => (
            <Tag key={role} color="blue">{role}</Tag>
          ))}
        </Space>
      ),
    },
    {
      title: '标签',
      dataIndex: 'labels',
      key: 'labels',
      width: 300,
      render: (labels: Record<string, string>) => (
        <Space wrap>
          {labels && Object.entries(labels).map(([key, value]) => (
            <Tag key={key} color="geekblue">
              {key}={value}
            </Tag>
          ))}
        </Space>
      ),
    },
    {
      title: '内部IP',
      dataIndex: 'internal_ip',
      key: 'internal_ip',
    },
    {
      title: '操作系统',
      dataIndex: 'os',
      key: 'os',
    },
    {
      title: '内核版本',
      dataIndex: 'kernel_version',
      key: 'kernel_version',
    },
    {
      title: '容器运行时',
      dataIndex: 'container_runtime',
      key: 'container_runtime',
    },
    {
      title: '操作',
      key: 'action',
      render: (_, record) => (
        <Space>
          <Button
            type="link"
            icon={<EditOutlined />}
            onClick={() => handleManageLabels(record)}
          >
            管理标签
          </Button>
        </Space>
      ),
    },
  ];

  return (
    <div>
      <ProTable<Node>
        headerTitle="节点列表"
        columns={columns}
        dataSource={tableData}
        loading={loading}
        rowKey="id"
        search={false}
        toolBarRender={() => [
          <Button
            type="primary"
            key="sync"
            onClick={handleSync}
            loading={loading}
          >
            同步节点
          </Button>,
        ]}
      />

      <Modal
        title={`管理节点标签 - ${editingNode?.name || ''}`}
        open={labelModalVisible}
        onCancel={() => {
          setLabelModalVisible(false);
          setEditingNode(null);
        }}
        onOk={handleSaveLabels}
        width={600}
      >
        <div style={{ marginBottom: 16 }}>
          <Button type="dashed" onClick={handleAddLabel} icon={<PlusOutlined />}>
            添加标签
          </Button>
        </div>
        {labels.map((label, index) => (
          <Space key={index} style={{ display: 'flex', marginBottom: 8 }}>
            <Input
              placeholder="标签键"
              value={label.key}
              onChange={(e) => handleLabelChange(index, 'key', e.target.value)}
              style={{ width: 200 }}
            />
            <Input
              placeholder="标签值"
              value={label.value}
              onChange={(e) => handleLabelChange(index, 'value', e.target.value)}
              style={{ width: 200 }}
            />
            <Popconfirm
              title="确认删除此标签?"
              onConfirm={() => handleRemoveLabel(index)}
            >
              <Button danger icon={<DeleteOutlined />} />
            </Popconfirm>
          </Space>
        ))}
        {labels.length === 0 && (
          <div style={{ color: '#999', textAlign: 'center', padding: '20px 0' }}>
            暂无标签，点击上方按钮添加
          </div>
        )}
      </Modal>
    </div>
  );
}
