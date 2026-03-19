import { useState } from 'react';
import { Card, Form, Input, Switch, Button, message, InputNumber } from 'antd';
import { ToolOutlined } from '@ant-design/icons';

export default function SystemConfig() {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);

  const onFinish = async (values: any) => {
    setLoading(true);
    try {
      // 这里调用保存系统配置的API
      message.success('系统配置保存成功');
    } catch (error) {
      message.error('保存失败，请稍后重试');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Card
      title={
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <ToolOutlined />
          系统配置
        </div>
      }
    >
      <Form
        form={form}
        layout="vertical"
        onFinish={onFinish}
        initialValues={{ 
          site_name: '物联网云平台',
          enable_registration: true,
          session_timeout: 24,
          max_login_attempts: 5,
          enable_captcha: false,
        }}
        style={{ maxWidth: 600 }}
      >
        <Form.Item
          name="site_name"
          label="站点名称"
          rules={[{ required: true, message: '请输入站点名称' }]}
        >
          <Input placeholder="请输入站点名称" />
        </Form.Item>

        <Form.Item
          name="site_description"
          label="站点描述"
        >
          <Input.TextArea rows={3} placeholder="请输入站点描述" />
        </Form.Item>

        <Form.Item
          name="enable_registration"
          label="允许用户注册"
          valuePropName="checked"
        >
          <Switch />
        </Form.Item>

        <Form.Item
          name="session_timeout"
          label="会话超时时间（小时）"
          rules={[{ required: true, message: '请输入会话超时时间' }]}
        >
          <InputNumber min={1} max={720} style={{ width: '100%' }} />
        </Form.Item>

        <Form.Item
          name="max_login_attempts"
          label="最大登录尝试次数"
          rules={[{ required: true, message: '请输入最大登录尝试次数' }]}
        >
          <InputNumber min={1} max={20} style={{ width: '100%' }} />
        </Form.Item>

        <Form.Item
          name="enable_captcha"
          label="登录启用验证码"
          valuePropName="checked"
        >
          <Switch />
        </Form.Item>

        <Form.Item>
          <Button type="primary" htmlType="submit" loading={loading}>
            保存配置
          </Button>
        </Form.Item>
      </Form>
    </Card>
  );
}
