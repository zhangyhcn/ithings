import type { MenuProps } from 'antd';
import { useEffect, useState } from 'react';
import { Outlet, useNavigate, useLocation } from 'react-router-dom';
import { Layout, Menu, Avatar, Dropdown, theme } from 'antd';
import {
  DashboardOutlined,
  ApartmentOutlined,
  EnvironmentOutlined,
  FolderOutlined,
  UserOutlined,
  LogoutOutlined,
  SettingOutlined,
  TeamOutlined,
  GlobalOutlined,
  ToolOutlined,
  ShoppingCartOutlined,
  LineChartOutlined,
  AimOutlined,
  BranchesOutlined,
  CalculatorOutlined,
  ShoppingOutlined,
  DollarOutlined,
  FileSearchOutlined,
  SolutionOutlined,
  FileTextOutlined,
  FileDoneOutlined,
  ContainerOutlined,
  LoginOutlined,
  LogoutOutlined as LogoutOutlinedIcon,
  AuditOutlined,
  SearchOutlined,
  SwapOutlined,
  RocketOutlined,
  SendOutlined,
  CarOutlined,
  RollbackOutlined,
  ImportOutlined,
  ExportOutlined,
  CheckCircleOutlined,
  ScheduleOutlined,
  BellOutlined,
  AccountBookOutlined,
  MoneyCollectOutlined,
  FilePdfOutlined,
  DatabaseOutlined,
  AppstoreOutlined,
  HomeOutlined,
} from '@ant-design/icons';
import { menuApi } from '@/services/api';
import type { Menu as MenuType } from '@/types';

const { Header, Sider, Content } = Layout;

interface User {
  id: string;
  username: string;
  email: string;
  role: string;
}

const iconMap: Record<string, React.ReactNode> = {
  DashboardOutlined: <DashboardOutlined />,
  ApartmentOutlined: <ApartmentOutlined />,
  EnvironmentOutlined: <EnvironmentOutlined />,
  FolderOutlined: <FolderOutlined />,
  UserOutlined: <UserOutlined />,
  TeamOutlined: <TeamOutlined />,
  SettingOutlined: <SettingOutlined />,
  GlobalOutlined: <GlobalOutlined />,
  ToolOutlined: <ToolOutlined />,
  ShoppingCartOutlined: <ShoppingCartOutlined />,
  LineChartOutlined: <LineChartOutlined />,
  AimOutlined: <AimOutlined />,
  BranchesOutlined: <BranchesOutlined />,
  CalculatorOutlined: <CalculatorOutlined />,
  ShoppingOutlined: <ShoppingOutlined />,
  DollarOutlined: <DollarOutlined />,
  FileSearchOutlined: <FileSearchOutlined />,
  SolutionOutlined: <SolutionOutlined />,
  FileTextOutlined: <FileTextOutlined />,
  FileDoneOutlined: <FileDoneOutlined />,
  ContainerOutlined: <ContainerOutlined />,
  LoginOutlined: <LoginOutlined />,
  LogoutOutlined: <LogoutOutlinedIcon />,
  AuditOutlined: <AuditOutlined />,
  SearchOutlined: <SearchOutlined />,
  SwapOutlined: <SwapOutlined />,
  RocketOutlined: <RocketOutlined />,
  SendOutlined: <SendOutlined />,
  CarOutlined: <CarOutlined />,
  RollbackOutlined: <RollbackOutlined />,
  ImportOutlined: <ImportOutlined />,
  ExportOutlined: <ExportOutlined />,
  CheckCircleOutlined: <CheckCircleOutlined />,
  ScheduleOutlined: <ScheduleOutlined />,
  BellOutlined: <BellOutlined />,
  AccountBookOutlined: <AccountBookOutlined />,
  MoneyCollectOutlined: <MoneyCollectOutlined />,
  FilePdfOutlined: <FilePdfOutlined />,
  DatabaseOutlined: <DatabaseOutlined />,
  AppstoreOutlined: <AppstoreOutlined />,
  HomeOutlined: <HomeOutlined />,
};

export default function BasicLayout() {
  const navigate = useNavigate();
  const location = useLocation();
  const [user, setUser] = useState<User | null>(null);
  const [menus, setMenus] = useState<MenuType[]>([]);
  const [selectedKeys, setSelectedKeys] = useState<string[]>([]);
  const [openKeys, setOpenKeys] = useState<string[]>([]);
  const {
    token: { colorBgContainer, borderRadiusLG },
  } = theme.useToken();

  useEffect(() => {
    const userStr = localStorage.getItem('user');
    if (userStr) {
      setUser(JSON.parse(userStr));
      loadMenus();
    } else {
      navigate('/user/login');
    }
  }, [navigate]);

  useEffect(() => {
    setSelectedKeys([location.pathname]);
    // 计算所有需要展开的父级路径
    const pathSegments = location.pathname.split('/').filter(Boolean);
    const newOpenKeys: string[] = [];
    let currentPath = '';
    for (let i = 0; i < pathSegments.length - 1; i++) {
      currentPath += '/' + pathSegments[i];
      newOpenKeys.push(currentPath);
    }
    setOpenKeys(newOpenKeys);
  }, [location.pathname]);

  const loadMenus = async () => {
    try {
      const data = await menuApi.getUserMenus();
      setMenus(data);
    } catch (error) {
      console.error('Failed to load menus:', error);
    }
  };

  const renderMenuItems = (menuList: MenuType[]): any[] => {
    return menuList.map((menu) => {
      const item: any = {
        key: menu.path,
        icon: menu.icon ? iconMap[menu.icon] : null,
        label: menu.name,
      };
      if (menu.children && menu.children.length > 0) {
        item.children = renderMenuItems(menu.children);
      }
      return item;
    });
  };

  const handleUserMenuClick = ({ key }: { key: string }) => {
    if (key === 'logout') {
      handleLogout();
    } else if (key === 'internationalization') {
      navigate('/settings/internationalization');
    } else {
      navigate(`/${key}`);
    }
  };

  const handleMenuClick = ({ key }: { key: string }) => {
    navigate(key);
  };

  const handleLogout = () => {
    localStorage.removeItem('access_token');
    localStorage.removeItem('refresh_token');
    localStorage.removeItem('user');
    localStorage.removeItem('tenant_id');
    navigate('/user/login');
  };

  const userMenu: MenuProps = {
    items: [
      {
        key: 'profile',
        icon: <UserOutlined />,
        label: '个人中心',
      },
      {
        key: 'settings',
        icon: <SettingOutlined />,
        label: '系统设置',
      },
      {
        key: 'internationalization',
        icon: <GlobalOutlined />,
        label: '国际化',
      },
      {
        type: 'divider',
      },
      {
        key: 'logout',
        icon: <LogoutOutlined />,
        label: '退出登录',
        danger: true,
      },
    ],
  };

  const menuItems = renderMenuItems(menus);

  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Sider
        theme="dark"
        breakpoint="lg"
        collapsedWidth="0"
        style={{
          overflow: 'auto',
          height: '100vh',
          position: 'fixed',
          left: 0,
          top: 0,
          bottom: 0,
        }}
      >
        <div style={{
          height: 64,
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          color: 'white',
          fontSize: 18,
          fontWeight: 'bold',
        }}>
          智能制造平台
        </div>
        <Menu
          theme="dark"
          mode="inline"
          selectedKeys={selectedKeys}
          openKeys={openKeys}
          items={menuItems}
          onClick={handleMenuClick}
          onOpenChange={setOpenKeys}
        />
      </Sider>
      <Layout style={{ marginLeft: 200 }}>
        <Header style={{
          padding: '0 24px',
          background: colorBgContainer,
          display: 'flex',
          justifyContent: 'flex-end',
          alignItems: 'center',
        }}>
          <Dropdown menu={{ ...userMenu, onClick: handleUserMenuClick }} placement="bottomRight">
            <div style={{ cursor: 'pointer', display: 'flex', alignItems: 'center', gap: 8 }}>
              <Avatar icon={<UserOutlined />} />
              <span>{user?.username || '用户'}</span>
            </div>
          </Dropdown>
        </Header>
        <Content style={{ margin: '24px 16px', padding: 24, minHeight: 280 }}>
          <div
            style={{
              padding: 24,
              minHeight: 360,
              background: colorBgContainer,
              borderRadius: borderRadiusLG,
            }}
          >
            <Outlet />
          </div>
        </Content>
      </Layout>
    </Layout>
  );
}
