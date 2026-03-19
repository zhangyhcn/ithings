import { useState } from 'react';
import { Card, Form, Select, Button, message } from 'antd';
import { GlobalOutlined } from '@ant-design/icons';

const { Option } = Select;

export default function Internationalization() {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);

  const onFinish = async (values: any) => {
    setLoading(true);
    try {
      // 这里调用保存国际化配置的API
      message.success('国际化配置保存成功');
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
          <GlobalOutlined />
          国际化设置
        </div>
      }
    >
      <Form
        form={form}
        layout="vertical"
        onFinish={onFinish}
        initialValues={{ language: 'zh-CN' }}
        style={{ maxWidth: 500 }}
      >
        <Form.Item
          name="language"
          label="默认语言"
          rules={[{ required: true, message: '请选择默认语言' }]}
        >
          <Select placeholder="请选择默认语言">
            <Option value="zh-CN">简体中文</Option>
            <Option value="en-US">English</Option>
            <Option value="ja-JP">日本語</Option>
            <Option value="ko-KR">한국어</Option>
          </Select>
        </Form.Item>

        <Form.Item
          name="timezone"
          label="时区"
          rules={[{ required: true, message: '请选择时区' }]}
        >
          <Select placeholder="请选择时区">
            <Option value="Asia/Shanghai">北京 (UTC+8)</Option>
            <Option value="UTC">UTC (UTC+0)</Option>
            <Option value="America/New_York">纽约 (UTC-5/UTC-4)</Option>
            <Option value="Europe/London">伦敦 (UTC±0/UTC+1)</Option>
            <Option value="Asia/Tokyo">东京 (UTC+9)</Option>
          </Select>
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
