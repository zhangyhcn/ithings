import { defineConfig } from '@umijs/max';

export default defineConfig({
  routes: [
    {
      path: '/user',
      routes: [
        {
          path: '/user/login',
          component: '@/pages/user/Login',
        },
        {
          path: '/user/register',
          component: '@/pages/user/Register',
        },
        {
          path: '/user/forgot-password',
          component: '@/pages/user/ForgotPassword',
        },
      ],
    },
    {
      path: '/create_tenant',
      component: '@/pages/tenant/Create',
    },
    {
      path: '/',
      component: '@/layouts/BasicLayout',
      routes: [
        {
          path: '/',
          redirect: '/dashboard',
        },
        {
          path: '/dashboard',
          component: '@/pages/Dashboard',
        },
        {
          path: '/tenant',
          component: '@/pages/tenant/List',
        },
        {
          path: '/users',
          component: '@/pages/user/List',
        },
        {
          path: '/site',
          component: '@/pages/site/List',
        },
        {
          path: '/namespace',
          component: '@/pages/namespace/List',
        },
        {
          path: '/organization',
          component: '@/pages/organization/List',
        },
        {
          path: '/department',
          component: '@/pages/department/List',
        },
        {
          path: '/resources',
          routes: [
            {
              path: '/resources/crd',
              component: '@/pages/resource/crd/List',
            },
            {
              path: '/resources/operator',
              component: '@/pages/resource/operator/List',
            },
            {
              path: '/resources/controller',
              component: '@/pages/resource/controller/List',
            },
          ],
        },
        {
          path: '/settings',
          routes: [
            {
              path: '/settings/system',
              component: '@/pages/settings/SystemConfig',
            },
            {
              path: '/settings/role',
              component: '@/pages/role/List',
            },
            {
              path: '/settings/user-role',
              component: '@/pages/user-role/List',
            },
            {
              path: '/settings/role-menu',
              component: '@/pages/role-menu/List',
            },
          ],
        },
        {
          path: '/device',
          routes: [
            {
              path: '/device/product',
              component: '@/pages/device/product/List',
            },
            {
              path: '/device/driver',
              component: '@/pages/device/driver/List',
            },
            {
              path: '/device/node',
              component: '@/pages/device/node/List',
            },
            {
              path: '/device/device',
              component: '@/pages/device/device/List',
            },
            {
              path: '/device/group',
              component: '@/pages/device/group/List',
            },
            {
              path: '/device/instance',
              component: '@/pages/device/instance/List',
            },
          ],
        },
        {
          path: '/config',
          routes: [
            {
              path: '/config/configmap',
              component: '@/pages/config/config_map/List',
            },
          ],
        },
      ],
    },
  ],
  npmClient: 'npm',
  proxy: {
    '/api': {
      target: 'http://localhost:8080',
      changeOrigin: true,
    },
  },
});