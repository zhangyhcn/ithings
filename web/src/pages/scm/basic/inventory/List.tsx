import React, { useState, useEffect } from 'react';
import { Table, Card, Input, Select, Space, message, Alert, Spin } from 'antd';
import { SearchOutlined } from '@ant-design/icons';
import { scmApi } from '@/services/api';
import { useScmOrg } from '@/hooks/useScmOrg';

interface Inventory {
  id: string;
  tenant_id: string;
  org_id: string;
  warehouse_id: string;
  warehouse_name?: string;
  material_id: string;
  material_code?: string;
  material_name?: string;
  batch_no?: string;
  quantity: number;
  frozen_qty: number;
  available_qty: number;
  cost_price?: number;
  production_date?: string;
  expiry_date?: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export default function InventoryList() {
  const [inventory, setInventory] = useState<Inventory[]>([]);
  const [loading, setLoading] = useState(false);
  const [searchText, setSearchText] = useState('');

  const { orgId, tenantId, loading: orgLoading, error: orgError } = useScmOrg();

  useEffect(() => {
    if (orgId) {
      loadInventory();
    }
  }, [orgId]);

  const loadInventory = async () => {
    setLoading(true);
    try {
      const data = await scmApi.listInventory(tenantId, orgId);
      setInventory(data);
    } catch (error) {
      message.error('加载库存列表失败');
    } finally {
      setLoading(false);
    }
  };

  const columns = [
    { title: '物料编码', dataIndex: 'material_code', key: 'material_code' },
    { title: '物料名称', dataIndex: 'material_name', key: 'material_name' },
    { title: '仓库', dataIndex: 'warehouse_name', key: 'warehouse_name' },
    { title: '批次号', dataIndex: 'batch_no', key: 'batch_no' },
    { title: '库存数量', dataIndex: 'quantity', key: 'quantity' },
    { title: '冻结数量', dataIndex: 'frozen_qty', key: 'frozen_qty' },
    { title: '可用数量', dataIndex: 'available_qty', key: 'available_qty' },
    { title: '成本价', dataIndex: 'cost_price', key: 'cost_price', render: (v: number) => v ? `¥${v.toFixed(2)}` : '-' },
    { title: '生产日期', dataIndex: 'production_date', key: 'production_date' },
    { title: '有效期', dataIndex: 'expiry_date', key: 'expiry_date' },
    { title: '状态', dataIndex: 'status', key: 'status' },
  ];

  const filteredData = inventory.filter(item =>
    Object.values(item).some(v => String(v).toLowerCase().includes(searchText.toLowerCase()))
  );

  return (
    <Card>
      <Space style={{ marginBottom: 16 }}>
        <Input.Search
          placeholder="搜索库存"
          allowClear
          onSearch={setSearchText}
          onChange={e => setSearchText(e.target.value)}
          style={{ width: 300 }}
          prefix={<SearchOutlined />}
        />
      </Space>
      <Table
        columns={columns}
        dataSource={filteredData}
        rowKey="id"
        loading={loading}
        scroll={{ x: 1200 }}
      />
    </Card>
  );
}
