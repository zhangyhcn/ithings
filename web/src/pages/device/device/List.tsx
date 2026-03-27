import { useState, useEffect } from 'react';
import { ProTable, ProColumns } from '@ant-design/pro-components';
import { Button, Modal, Form, Input, message, Space, Popconfirm, Select } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { deviceApi, productApi, driverApi } from '@/services/api';
import type { Device, Product, Driver } from '@/types';

const { TextArea } = Input;

export default function DeviceList() {
  const [tableData, setTableData] = useState<Device[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<Device | null>(null);
  const [tenantId, setTenantId] = useState<string>('');
  const [form] = Form.useForm();
  const [products, setProducts] = useState<Product[]>([]);
  const [drivers, setDrivers] = useState<Driver[]>([]);

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
      const [deviceData, productData, driverData] = await Promise.all([
        deviceApi.list(user.tenant_id),
        productApi.list(user.tenant_id),
        driverApi.list(user.tenant_id),
      ]);
      setTableData(Array.isArray(deviceData) ? deviceData : []);
      setProducts(Array.isArray(productData) ? productData : []);
      setDrivers(Array.isArray(driverData) ? driverData : []);
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
    if (!tenantId) return;
    try {
      const payload = {
        ...values,
        device_profile: values.device_profile ? JSON.parse(values.device_profile) : {},
      };
      if (editingRecord) {
        await deviceApi.update(tenantId, editingRecord.id, payload);
        message.success('更新成功');
      } else {
        await deviceApi.create(tenantId, payload);
        message.success('创建成功');
      }
      setModalVisible(false);
      form.resetFields();
      setEditingRecord(null);
      fetchData();
    } catch (error) {
      message.error('操作失败');
      console.error(error);
    }
  };

  const handleEdit = (record: Device) => {
    setEditingRecord(record);
    form.setFieldsValue({
      ...record,
      device_profile: JSON.stringify(record.device_profile, null, 2),
    });
    setModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    if (!tenantId) return;
    try {
      await deviceApi.delete(tenantId, id);
      message.success('删除成功');
      fetchData();
    } catch (error) {
      console.error(error);
    }
  };

  const columns: ProColumns<Device>[] = [
    {
      title: '设备名称',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: '设备型号',
      dataIndex: 'model',
      key: 'model',
    },
    {
      title: '生产厂家',
      dataIndex: 'manufacturer',
      key: 'manufacturer',
    },
    {
      title: '设备镜像',
      dataIndex: 'device_image',
      key: 'device_image',
      ellipsis: true,
    },
    {
      title: '驱动镜像',
      dataIndex: 'driver_image',
      key: 'driver_image',
      ellipsis: true,
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      valueEnum: {
        active: { text: '正常', status: 'Success' },
        inactive: { text: '停用', status: 'Default' },
        fault: { text: '故障', status: 'Error' },
      },
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
        headerTitle="设备列表"
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
            创建设备
          </Button>,
        ]}
      />

      <Modal
        title={editingRecord ? '编辑设备' : '创建设备'}
        open={modalVisible}
        onCancel={() => {
          setModalVisible(false);
          form.resetFields();
          setEditingRecord(null);
        }}
        onOk={form.submit}
        width={800}
      >
        <Form form={form} layout="vertical" onFinish={handleAdd}>
          <Form.Item
            name="name"
            label="设备名称"
            rules={[{ required: true, message: '请输入设备名称' }]}
          >
            <Input placeholder="请输入设备名称" />
          </Form.Item>
          <Form.Item name="product_id" label="所属产品">
            <Select
              placeholder="请选择产品"
              allowClear
              showSearch
              optionFilterProp="children"
            >
              {products.map((product) => (
                <Select.Option key={product.id} value={product.id}>
                  {product.name}
                </Select.Option>
              ))}
            </Select>
          </Form.Item>
          <Form.Item name="model" label="设备型号">
            <Input placeholder="请输入设备型号规格" />
          </Form.Item>
          <Form.Item name="manufacturer" label="生产厂家">
            <Input placeholder="请输入生产厂家" />
          </Form.Item>
          <Form.Item name="device_image" label="设备镜像" rules={[{ required: true, message: '请输入设备镜像地址' }]}>
            <Input placeholder="例如: device-meter:latest" />
          </Form.Item>
          <Form.Item name="driver_image" label="驱动镜像">
            <Select
              placeholder="请选择驱动镜像"
              allowClear
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
          <Form.Item name="description" label="描述">
            <Input.TextArea placeholder="请输入描述" rows={2} />
          </Form.Item>
          <Form.Item
            name="device_profile"
            label="设备配置文件 (JSON)"
            extra="定义设备的连接参数、数据点等配置"
          >
            <TextArea
              rows={8}
              placeholder='{"connection": {}, "dataPoints": []}'
            />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
}
