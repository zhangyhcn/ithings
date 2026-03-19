import { Outlet } from 'umi';
import { Layout } from 'antd';

const { Content } = Layout;

export default function DeviceLayout() {
  return (
    <Layout>
      <Content>
        <Outlet />
      </Content>
    </Layout>
  );
}
