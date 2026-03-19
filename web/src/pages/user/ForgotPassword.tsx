import { useState } from 'react';
import { Form, Input, Button, Card, message } from 'antd';
import { UserOutlined, LockOutlined, MailOutlined } from '@ant-design/icons';
import { useNavigate } from 'react-router-dom';
import { authApi } from '@/services/api';

const { Option } = Select;

interface ForgotPasswordFormValues {
  email: string;
}

export default function ForgotPassword() {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);

  const onFinish = async (values: ForgotPasswordFormValues) => {
    setLoading(true);
    try {
      await authApi.forgotPassword(values);
      message.success('重置密码邮件已发送，请检查邮箱');
      navigate('/user/login');
    } catch (error) {
      console.error(error);
      message.error('发送失败，请稍后重试');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{
      display: 'flex',
      justifyContent: 'center',
      alignItems: 'center',
      minHeight: '100vh',
      background: '#f0f2f5',
    }}>
      <Card
        title={<h2 style={{ textAlign: 'center', margin: 0 }}>忘记密码</h2>}
        style={{ width: 400 }}
      >
        <Form
          name="forgot-password"
          onFinish={onFinish}
          autoComplete="off"
          size="large"
        >
          <Form.Item
            name="email"
            rules={[
              { required: true, message: '请输入邮箱' },
              { type: 'email', message: '请输入有效的邮箱地址' }
            ]}
          >
            <Input
              prefix={<MailOutlined />}
              placeholder="邮箱地址"
            />
          </Form.Item>

          <Form.Item>
            <Button type="primary" htmlType="submit" block loading={loading}>
              发送重置邮件
            </Button>
          </Form.Item>

          <Form.Item>
            <Button type="link" block onClick={() => navigate('/user/login')}>
              返回登录
            </Button>
          </Form.Item>
        </Form>
      </Card>
    </div>
  );
}
