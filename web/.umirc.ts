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
        {
          path: '/scm',
          routes: [
            // 计划与预测
            {
              path: '/scm/plan',
              routes: [
                { path: '/scm/plan/demand', component: '@/pages/scm/plan/demand/List' },
                { path: '/scm/plan/supply', component: '@/pages/scm/plan/supply/List' },
                { path: '/scm/plan/mrp', component: '@/pages/scm/plan/mrp/List' },
              ],
            },
            // 采购管理
            {
              path: '/scm/procurement',
              routes: [
                { path: '/scm/procurement/supplier', component: '@/pages/scm/procurement/supplier/List' },
                { path: '/scm/procurement/quotation', component: '@/pages/scm/procurement/quotation/List' },
                { path: '/scm/procurement/bidding', component: '@/pages/scm/procurement/bidding/List' },
                { path: '/scm/procurement/contract', component: '@/pages/scm/procurement/contract/List' },
                { path: '/scm/procurement/order', component: '@/pages/scm/procurement/order/List' },
              ],
            },
            // 生产制造
            {
              path: '/scm/production',
              routes: [
                { path: '/scm/production/bom', component: '@/pages/scm/production/bom/List' },
                { path: '/scm/production/order', component: '@/pages/scm/production/order/List' },
                { path: '/scm/production/route', component: '@/pages/scm/production/route/List' },
              ],
            },
            // 仓储物流
            {
              path: '/scm/warehouse',
              routes: [
                { path: '/scm/warehouse/inbound', component: '@/pages/scm/warehouse/inbound/List' },
                { path: '/scm/warehouse/outbound', component: '@/pages/scm/warehouse/outbound/List' },
                { path: '/scm/warehouse/stocktaking', component: '@/pages/scm/warehouse/stocktaking/List' },
                { path: '/scm/warehouse/inventory', component: '@/pages/scm/warehouse/inventory/List' },
                { path: '/scm/warehouse/transfer', component: '@/pages/scm/warehouse/transfer/List' },
              ],
            },
            // 订单交付
            {
              path: '/scm/delivery',
              routes: [
                { path: '/scm/delivery/sales', component: '@/pages/scm/delivery/sales/List' },
                { path: '/scm/delivery/shipment', component: '@/pages/scm/delivery/shipment/List' },
                { path: '/scm/delivery/logistics', component: '@/pages/scm/delivery/logistics/List' },
              ],
            },
            // 退货管理
            {
              path: '/scm/return',
              routes: [
                { path: '/scm/return/purchase', component: '@/pages/scm/return/purchase/List' },
                { path: '/scm/return/sales', component: '@/pages/scm/return/sales/List' },
                { path: '/scm/return/audit', component: '@/pages/scm/return/audit/List' },
              ],
            },
            // 协同管理
            {
              path: '/scm/collab',
              routes: [
                { path: '/scm/collab/task', component: '@/pages/scm/collab/task/List' },
                { path: '/scm/collab/message', component: '@/pages/scm/collab/message/List' },
                { path: '/scm/collab/document', component: '@/pages/scm/collab/document/List' },
              ],
            },
            // 财务结算
            {
              path: '/scm/finance',
              routes: [
                { path: '/scm/finance/payable', component: '@/pages/scm/finance/payable/List' },
                { path: '/scm/finance/cost', component: '@/pages/scm/finance/cost/List' },
                { path: '/scm/finance/invoice', component: '@/pages/scm/finance/invoice/List' },
              ],
            },
            // 基础数据
            {
              path: '/scm/basic',
              routes: [
                { path: '/scm/basic/material', component: '@/pages/scm/basic/material/List' },
                { path: '/scm/basic/warehouse', component: '@/pages/scm/basic/warehouse/List' },
              ],
            },
          ],
        },
        {
          path: '/finance',
          routes: [
            // 财务会计模块
            {
                      {
          path: '/finance/accounting',
          routes: [
            { path: '/finance/accounting/general-ledger', component: '@/pages/finance/accounting/general-ledger/List' },
            { path: '/finance/accounting/chart-of-accounts', component: '@/pages/finance/accounting/chart-of-accounts/List' },
            { path: '/finance/accounting/voucher', component: '@/pages/finance/accounting/voucher/List' },
            { path: '/finance/accounting/receivable', component: '@/pages/finance/accounting/receivable/List' },
            { path: '/finance/accounting/payable', component: '@/pages/finance/accounting/payable/List' },
            { path: '/finance/accounting/asset', component: '@/pages/finance/accounting/asset/List' },
          ],
        },
            // 管理会计模块
            {
              path: '/finance/management',
              routes: [
                { path: '/finance/management/cost', component: '@/pages/finance/management/cost/List' },
                { path: '/finance/management/budget', component: '@/pages/finance/management/budget/List' },
                { path: '/finance/management/fund', component: '@/pages/finance/management/fund/List' },
              ],
            },
            // 运营支持模块
            {
              path: '/finance/operation',
              routes: [
                { path: '/finance/operation/cash', component: '@/pages/finance/operation/cash/List' },
                { path: '/finance/operation/expense', component: '@/pages/finance/operation/expense/List' },
                { path: '/finance/operation/invoice', component: '@/pages/finance/operation/invoice/List' },
              ],
            },
            // 财务报告与合规模块
            {
              path: '/finance/report',
              routes: [
                { path: '/finance/report/statement', component: '@/pages/finance/report/statement/List' },
                { path: '/finance/report/tax', component: '@/pages/finance/report/tax/List' },
                { path: '/finance/report/consolidation', component: '@/pages/finance/report/consolidation/List' },
              ],
            },
          ],
        },
      ],
    },
  ],
  npmClient: 'npm',
  proxy: {
    '/api/v1/scm': {
      target: 'http://localhost:8081',
      changeOrigin: true,
    },
    '/api/v1/finance': {
      target: 'http://localhost:8082',
      changeOrigin: true,
    },
    '/api': {
      target: 'http://localhost:9005',
      changeOrigin: true,
    },
  },
});