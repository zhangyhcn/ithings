import React, { useState, useEffect } from 'react';
import { Table, Button, Space, Modal, Form, Input, InputNumber, Select, message, Popconfirm, Card, Alert, Spin } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { scmApi } from '@/services/api';
import { useScmOrg } from '@/hooks/useScmOrg';

interface Material {
  id: string;
  tenant_id: string;
  org_id: string;
  code: string;
  name: string;
  specification?: string;
  model?: string;
  unit: string;
  barcode?: string;
  status: string;
  safety_stock: number;
  max_stock: number;
  min_order_qty: number;
  lead_time: number;
  purchase_price?: number;
  sale_price?: number;
  cost_price?: number;
  remark?: string;
  created_at: string;
  updated_at: string;
}

export default function MaterialList() {
  const [materials, setMaterials] = useState<Material[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingMaterial, setEditingMaterial] = useState<Material | null>(null);
  const [form] = Form.useForm();

  const { orgId, tenantId, loading: orgLoading, error: orgError } = useScmOrg();

  useEffect(() => {
    if (orgId) {
      loadMaterials();
    }
  }, [orgId]);

  const loadMaterials = async () => {
    setLoading(true);
    try {
      const data = await scmApi.listMaterials(tenantId, orgId);
      setMaterials(data);
    } catch (error) {
      message.error('加载物料列表失败');
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = () => {
    setEditingMaterial(null);
    form.resetFields();
    setModalVisible(true);
  };

  const handleEdit = (material: Material) => {
    setEditingMaterial(material);
    form.setFieldsValue(material);
    setModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    try {
      await scmApi.deleteMaterial(tenantId, orgId, id);
      message.success('删除成功');
      loadMaterials();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      if (editingMaterial) {
        await scmApi.updateMaterial(tenantId, orgId, editingMaterial.id, values);
        message.success('更新成功');
      } else {
        await scmApi.createMaterial(tenantId, orgId, values);
        message.success('创建成功');
      }
      setModalVisible(false);
      loadMaterials();
    } catch (error) {
      message.error('操作失败');
    }
  };

  const columns = [
    { title: '物料编码', dataIndex: 'code', key: 'code' },
    { title: '物料名称', dataIndex: 'name', key: 'name' },
    { title: '规格', dataIndex: 'specification', key: 'specification' },
    { title: '型号', dataIndex: 'model', key: 'model' },
    { title: '单位', dataIndex: 'unit', key: 'unit' },
    { title: '安全库存', dataIndex: 'safety_stock', key: 'safety_stock' },
    { title: '采购价', dataIndex: 'purchase_price', key: 'purchase_price', render: (v: number) => v ? `¥${v.toFixed(2)}` : '-' },
    { title: '状态', dataIndex: 'status', key: 'status' },
    {
      title: '操作',
      key: 'action',
      render: (_: any, record: Material) => (
        <Space>
          <Button type="link" icon={<EditOutlined />} onClick={() => handleEdit(record)}>
            编辑
          </Button>
          <Popconfirm title="确认删除?" onConfirm={() => handleDelete(record.id)}>
            <Button type="link" danger icon={<DeleteOutlined />}>
              删除
            </Button>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <Card>
      <Space style={{ marginBottom: 16 }}>
        <Button type="primary" icon={<PlusOutlined />} onClick={handleCreate}>
          新增物料
        </Button>
      </Space>
      <Table columns={columns} dataSource={materials} rowKey="id" loading={loading} />
      <Modal
        title={editingMaterial ? '编辑物料' : '新增物料'}
        open={modalVisible}
        onOk={handleSubmit}
        onCancel={() => setModalVisible(false)}
        width={800}
      >
        <Form form={form} layout="vertical">
          <Form.Item name="code" label="物料编码" rules={[{ required: true }]}>
            <Input disabled={!!editingMaterial} />
          </Form.Item>
          <Form.Item name="name" label="物料名称" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="specification" label="规格">
            <Input />
          </Form.Item>
          <Form.Item name="model" label="型号">
            <Input />
          </Form.Item>
          <Form.Item name="unit" label="单位" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="safety_stock" label="安全库存" initialValue={0}>
            <InputNumber min={0} />
          </Form.Item>
          <Form.Item name="purchase_price" label="采购价">
            <InputNumber min={0} precision={2} />
          </Form.Item>
          <Form.Item name="sale_price" label="销售价">
            <InputNumber min={0} precision={2} />
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <Input.TextArea />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
}
