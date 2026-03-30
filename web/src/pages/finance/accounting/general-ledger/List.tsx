import React, { useEffect, useState } from 'react';
import { Card, Table, Button, Space, DatePicker, message } from 'antd';
import { ReloadOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { useScmOrg } from '@/hooks/useScmOrg';

interface LedgerAccount {
  id: string;
  account_code: string;
  account_name: string;
  account_type: string;
  level: number;
  debit_credit: string;
  debit_amount: number;
  credit_amount: number;
  balance: number;
  last_transaction_date: string;
}

export default function GeneralLedgerList() {
  const [accounts, setAccounts] = useState<LedgerAccount[]>([]);
  const [loading, setLoading] = useState(false);
  const [dateRange, setDateRange] = useState<string>('2026-01-01,2026-03-31');
  
  const { orgId, tenantId, loading: orgLoading, error: orgError } = useScmOrg();

  useEffect(() => {
    if (tenantId && orgId) {
      loadLedgerData();
    }
  }, [tenantId, orgId]);

  const loadLedgerData = async () => {
    if (!tenantId || !orgId) return;
    
    setLoading(true);
    try {
      // 模拟总账数据（实际应该从后端获取科目余额汇总）
      const mockData: LedgerAccount[] = [
        {
          id: 'acc-001',
          account_code: '1001',
          account_name: '库存现金',
          account_type: 'asset',
          level: 1,
          debit_credit: 'debit',
          debit_amount: 150000.00,
          credit_amount: 120000.00,
          balance: 30000.00,
          last_transaction_date: '2026-03-30',
        },
        {
          id: 'acc-002',
          account_code: '1002',
          account_name: '银行存款',
          account_type: 'asset',
          level: 1,
          debit_credit: 'debit',
          debit_amount: 850000.00,
          credit_amount: 420000.00,
          balance: 430000.00,
          last_transaction_date: '2026-03-30',
        },
        {
          id: 'acc-003',
          account_code: '1122',
          account_name: '应收账款',
          account_type: 'asset',
          level: 1,
          debit_credit: 'debit',
          debit_amount: 650000.00,
          credit_amount: 380000.00,
          balance: 270000.00,
          last_transaction_date: '2026-03-30',
        },
        {
          id: 'acc-004',
          account_code: '2202',
          account_name: '应付账款',
          account_type: 'liability',
          level: 1,
          debit_credit: 'credit',
          debit_amount: 320000.00,
          credit_amount: 580000.00,
          balance: 260000.00,
          last_transaction_date: '2026-03-30',
        },
        {
          id: 'acc-005',
          account_code: '6001',
          account_name: '主营业务收入',
          account_type: 'income',
          level: 1,
          debit_credit: 'credit',
          debit_amount: 0.00,
          credit_amount: 1200000.00,
          balance: 1200000.00,
          last_transaction_date: '2026-03-30',
        },
      ];
      
      setAccounts(mockData);
    } catch (error) {
      message.error('加载总账数据失败');
    } finally {
      setLoading(false);
    }
  };

  const columns: ColumnsType<LedgerAccount> = [
    {
      title: '科目代码',
      dataIndex: 'account_code',
      key: 'account_code',
      width: 120,
    },
    {
      title: '科目名称',
      dataIndex: 'account_name',
      key: 'account_name',
      width: 200,
    },
    {
      title: '科目类型',
      dataIndex: 'account_type',
      key: 'account_type',
      width: 100,
      render: (type) => {
        const typeMap: Record<string, string> = {
          asset: '资产',
          liability: '负债',
          equity: '权益',
          income: '收入',
          expense: '费用',
        };
        return typeMap[type] || type;
      },
    },
    {
      title: '借方发生额',
      dataIndex: 'debit_amount',
      key: 'debit_amount',
      width: 130,
      align: 'right' as const,
      render: (amount) => amount.toFixed(2),
    },
    {
      title: '贷方发生额',
      dataIndex: 'credit_amount',
      key: 'credit_amount',
      width: 130,
      align: 'right' as const,
      render: (amount) => amount.toFixed(2),
    },
    {
      title: '余额',
      dataIndex: 'balance',
      key: 'balance',
      width: 130,
      align: 'right' as const,
      render: (amount) => amount.toFixed(2),
    },
    {
      title: '方向',
      dataIndex: 'debit_credit',
      key: 'debit_credit',
      width: 80,
      render: (dc) => dc === 'debit' ? '借' : '贷',
    },
    {
      title: '最后交易日期',
      dataIndex: 'last_transaction_date',
      key: 'last_transaction_date',
      width: 120,
    },
  ];

  return (
    <Card
      title="总账管理"
      extra={
        <Space>
          <DatePicker.RangePicker 
            format="YYYY-MM-DD"
            defaultValue={[]}
            onChange={(dates) => {
              if (dates && dates[0] && dates[1]) {
                setDateRange(`${dates[0].format('YYYY-MM-DD')},${dates[1].format('YYYY-MM-DD')}`);
              }
            }}
            placeholder={['开始日期', '结束日期']}
          />
          <Button
            type="primary"
            icon={<ReloadOutlined />}
            onClick={loadLedgerData}
            disabled={orgLoading || !!orgError}
          >
            刷新
          </Button>
        </Space>
      }
    >
      {orgLoading ? (
        <div style={{ textAlign: 'center', padding: '50px' }}>加载组织信息...</div>
      ) : orgError ? (
        <div style={{ textAlign: 'center', padding: '50px', color: 'red' }}>{orgError}</div>
      ) : (
        <Table
          columns={columns}
          dataSource={accounts}
          rowKey="id"
          loading={loading}
          pagination={{
            pageSize: 20,
            showSizeChanger: true,
            showTotal: (total) => `共 ${total} 条记录`,
          }}
          summary={(pageData) => {
            let totalDebit = 0;
            let totalCredit = 0;
            pageData.forEach(({ debit_amount, credit_amount }) => {
              totalDebit += debit_amount;
              totalCredit += credit_amount;
            });
            return (
              <Table.Summary.Row>
                <Table.Summary.Cell index={0} colSpan={3}>
                  <strong>合计</strong>
                </Table.Summary.Cell>
                <Table.Summary.Cell index={3} align="right">
                  <strong>{totalDebit.toFixed(2)}</strong>
                </Table.Summary.Cell>
                <Table.Summary.Cell index={4} align="right">
                  <strong>{totalCredit.toFixed(2)}</strong>
                </Table.Summary.Cell>
                <Table.Summary.Cell index={5} colSpan={2}></Table.Summary.Cell>
              </Table.Summary.Row>
            );
          }}
        />
      )}
    </Card>
  );
}
