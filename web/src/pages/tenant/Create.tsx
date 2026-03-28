import { useState } from 'react';
import { Form, Input, Button, Card, message, InputNumber } from 'antd';
import { useNavigate } from 'react-router-dom';
import { tenantApi } from '@/services/api';

interface CreateTenantFormValues {
  name: string;
  slug: string;
  description: string;
  registry_url?: string;
  virtual_cluster_name?: string;
  remote_transport_config?: string;
  admin_username: string;
  admin_email: string;
  admin_password: string;
  admin_confirm_password: string;
}

export default function CreateTenant() {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);
  const [form] = Form.useForm();

  const onFinish = async (values: CreateTenantFormValues) => {
    if (values.admin_password !== values.admin_confirm_password) {
      message.error('两次输入的管理员密码不一致');
      return;
    }
    setLoading(true);
    try {
      let remote_transport: any = undefined;
      if (values.remote_transport_config && values.remote_transport_config.trim()) {
        try {
          remote_transport = JSON.parse(values.remote_transport_config);
        } catch (e) {
          message.error('远程传输配置JSON格式错误');
          setLoading(false);
          return;
        }
      }

      const config = {
        registry_url: values.registry_url,
        virtual_cluster_name: values.virtual_cluster_name,
        remote_transport,
      };
      await tenantApi.create({
        name: values.name,
        slug: values.slug,
        description: values.description,
        admin_username: values.admin_username,
        admin_email: values.admin_email,
        admin_password: values.admin_password,
        config,
      });
      message.success('租户创建成功，管理员用户已创建');
      navigate('/tenant');
    } catch (error) {
      console.error(error);
      message.error('创建失败，请检查输入信息');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ padding: '24px', maxWidth: '800px', margin: '0 auto' }}>
      <Card title="创建租户" bordered={false}>
        <Form
          form={form}
          layout="vertical"
          onFinish={onFinish}
          autoComplete="off"
        >
          <div style={{ marginBottom: '24px' }}>
            <h3>租户信息</h3>
          </div>

          <Form.Item
            name="name"
            label="租户名称"
            rules={[{ required: true, message: '请输入租户名称' }]}
          >
            <Input placeholder="请输入租户名称" />
          </Form.Item>

          <Form.Item
            name="slug"
            label="租户标识"
            rules={[
              { required: true, message: '请输入租户标识' },
              { pattern: /^[a-z0-9-]+$/, message: '只能包含小写字母、数字和连字符' },
            ]}
          >
            <Input placeholder="请输入租户标识 (如: company-a)" />
          </Form.Item>

          <Form.Item name="description" label="租户描述">
            <Input.TextArea 
              placeholder="请输入租户描述" 
              rows={4}
            />
          </Form.Item>

          <Form.Item name="registry_url" label="镜像仓库地址">
            <Input placeholder="如: https://registry.example.com" />
          </Form.Item>

          <Form.Item name="virtual_cluster_name" label="虚拟集群名">
            <Input placeholder="虚拟集群名称" />
          </Form.Item>

          <div style={{ marginBottom: '24px', marginTop: '32px' }}>
            <h3>远程传输配置</h3>
          </div>

          <Form.Item name="remote_transport_config" label="远程传输配置(JSON)">
            <Input.TextArea 
              placeholder={`示例 (MQTT):
{
  "type": "mqtt",
  "broker": "tcp://localhost:1883",
  "username": "user",
  "password": "pass",
  "client_id": "client-id"
}

示例 (Kafka):
{
  "type": "kafka",
  "brokers": "localhost:9092",
  "username": "user",
  "password": "pass"
}`} 
              rows={12}
            />
          </Form.Item>

          <div style={{ marginBottom: '24px', marginTop: '32px' }}>
            <h3>管理员信息</h3>
          </div>

          <Form.Item
            name="admin_username"
            label="管理员用户名"
            rules={[
              { required: true, message: '请输入管理员用户名' },
              { min: 3, message: '用户名至少3个字符' },
            ]}
          >
            <Input placeholder="请输入管理员用户名" />
          </Form.Item>

          <Form.Item
            name="admin_email"
            label="管理员邮箱"
            rules={[
              { required: true, message: '请输入管理员邮箱' },
              { type: 'email', message: '请输入正确的邮箱格式' },
            ]}
          >
            <Input placeholder="请输入管理员邮箱" />
          </Form.Item>

          <Form.Item
            name="admin_password"
            label="管理员密码"
            rules={[
              { required: true, message: '请输入管理员密码' },
              { min: 6, message: '密码至少6位' },
            ]}
          >
            <Input.Password placeholder="请输入管理员密码" />
          </Form.Item>

          <Form.Item
            name="admin_confirm_password"
            label="确认密码"
            dependencies={['admin_password']}
            rules={[
              { required: true, message: '请确认管理员密码' },
              ({ getFieldValue }) => ({
                validator(_, value) {
                  if (!value || getFieldValue('admin_password') === value) {
                    return Promise.resolve();
                  }
                  return Promise.reject(new Error('两次输入的密码不一致'));
                },
              }),
            ]}
          >
            <Input.Password placeholder="请再次输入管理员密码" />
          </Form.Item>

          <Form.Item>
            <Button type="primary" htmlType="submit" loading={loading} size="large">
              创建租户
            </Button>
            <Button 
              style={{ marginLeft: '8px' }} 
              onClick={() => navigate('/tenant')}
              size="large"
            >
              取消
            </Button>
          </Form.Item>
        </Form>
      </Card>
    </div>
  );
}