-- 企业财务管理系统 (EFMS) 数据库表结构
-- 核心模块一：财务会计模块

-- ========================================
-- 1. 总账管理
-- ========================================

-- 会计科目表
CREATE TABLE IF NOT EXISTS accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    account_code VARCHAR(50) NOT NULL,
    account_name VARCHAR(200) NOT NULL,
    account_type VARCHAR(20) NOT NULL, -- asset, liability, equity, income, expense
    parent_id UUID,
    level INTEGER NOT NULL DEFAULT 1,
    is_leaf BOOLEAN NOT NULL DEFAULT true,
    debit_credit VARCHAR(10) NOT NULL, -- debit, credit
    currency VARCHAR(10) DEFAULT 'CNY',
    status VARCHAR(20) DEFAULT 'active',
    remarks TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, account_code)
);

-- 凭证表
CREATE TABLE IF NOT EXISTS vouchers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    voucher_no VARCHAR(50) NOT NULL,
    voucher_date DATE NOT NULL,
    voucher_type VARCHAR(20) NOT NULL, -- receipt, payment, transfer
    description TEXT,
    total_debit DECIMAL(15,2) NOT NULL DEFAULT 0,
    total_credit DECIMAL(15,2) NOT NULL DEFAULT 0,
    status VARCHAR(20) DEFAULT 'draft', -- draft, submitted, approved, posted
    created_by UUID,
    approved_by UUID,
    approved_at TIMESTAMP,
    posted_by UUID,
    posted_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, voucher_no)
);

-- 凭证明细表
CREATE TABLE IF NOT EXISTS voucher_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    voucher_id UUID NOT NULL,
    account_id UUID NOT NULL,
    description TEXT,
    debit_amount DECIMAL(15,2) NOT NULL DEFAULT 0,
    credit_amount DECIMAL(15,2) NOT NULL DEFAULT 0,
    currency VARCHAR(10) DEFAULT 'CNY',
    exchange_rate DECIMAL(10,6) DEFAULT 1,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- ========================================
-- 2. 应收账款管理
-- ========================================

-- 应收账款表
CREATE TABLE IF NOT EXISTS receivables (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    receivable_no VARCHAR(50) NOT NULL,
    customer_id UUID NOT NULL,
    customer_name VARCHAR(200) NOT NULL,
    invoice_no VARCHAR(100),
    invoice_date DATE,
    receivable_date DATE NOT NULL,
    due_date DATE NOT NULL,
    currency VARCHAR(10) DEFAULT 'CNY',
    original_amount DECIMAL(15,2) NOT NULL DEFAULT 0,
    received_amount DECIMAL(15,2) NOT NULL DEFAULT 0,
    outstanding_amount DECIMAL(15,2) GENERATED ALWAYS AS (original_amount - received_amount) STORED,
    status VARCHAR(20) DEFAULT 'open', -- open, partial, closed, overdue
    remarks TEXT,
    created_by UUID,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, receivable_no)
);

-- 收款记录表
CREATE TABLE IF NOT EXISTS receipts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    receipt_no VARCHAR(50) NOT NULL,
    receivable_id UUID NOT NULL,
    receipt_date DATE NOT NULL,
    amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(10) DEFAULT 'CNY',
    payment_method VARCHAR(50), -- cash, bank_transfer, check, etc.
    bank_account VARCHAR(100),
    remarks TEXT,
    created_by UUID,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, receipt_no)
);

-- ========================================
-- 3. 应付账款管理
-- ========================================

-- 应付账款表
CREATE TABLE IF NOT EXISTS payables (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    payable_no VARCHAR(50) NOT NULL,
    supplier_id UUID NOT NULL,
    supplier_name VARCHAR(200) NOT NULL,
    invoice_no VARCHAR(100),
    invoice_date DATE,
    payable_date DATE NOT NULL,
    due_date DATE NOT NULL,
    currency VARCHAR(10) DEFAULT 'CNY',
    original_amount DECIMAL(15,2) NOT NULL DEFAULT 0,
    paid_amount DECIMAL(15,2) NOT NULL DEFAULT 0,
    outstanding_amount DECIMAL(15,2) GENERATED ALWAYS AS (original_amount - paid_amount) STORED,
    status VARCHAR(20) DEFAULT 'open', -- open, partial, closed, overdue
    remarks TEXT,
    created_by UUID,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, payable_no)
);

-- 付款记录表
CREATE TABLE IF NOT EXISTS payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    payment_no VARCHAR(50) NOT NULL,
    payable_id UUID NOT NULL,
    payment_date DATE NOT NULL,
    amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(10) DEFAULT 'CNY',
    payment_method VARCHAR(50),
    bank_account VARCHAR(100),
    remarks TEXT,
    created_by UUID,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, payment_no)
);

-- ========================================
-- 4. 固定资产管理
-- ========================================

-- 固定资产表
CREATE TABLE IF NOT EXISTS assets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    asset_no VARCHAR(50) NOT NULL,
    asset_name VARCHAR(200) NOT NULL,
    asset_category VARCHAR(100),
    asset_type VARCHAR(50), -- machinery, vehicle, building, equipment, etc.
    acquisition_date DATE NOT NULL,
    acquisition_cost DECIMAL(15,2) NOT NULL,
    salvage_value DECIMAL(15,2) DEFAULT 0,
    useful_life_years INTEGER NOT NULL,
    depreciation_method VARCHAR(20) DEFAULT 'straight_line', -- straight_line, declining_balance, etc.
    accumulated_depreciation DECIMAL(15,2) DEFAULT 0,
    net_book_value DECIMAL(15,2) GENERATED ALWAYS AS (acquisition_cost - accumulated_depreciation) STORED,
    location VARCHAR(200),
    department VARCHAR(100),
    responsible_person VARCHAR(100),
    status VARCHAR(20) DEFAULT 'active', -- active, disposed, transferred
    remarks TEXT,
    created_by UUID,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, asset_no)
);

-- 资产折旧记录表
CREATE TABLE IF NOT EXISTS asset_depreciations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    asset_id UUID NOT NULL,
    depreciation_date DATE NOT NULL,
    depreciation_amount DECIMAL(15,2) NOT NULL,
    accumulated_depreciation DECIMAL(15,2) NOT NULL,
    voucher_id UUID,
    remarks TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- ========================================
-- 核心模块二：管理会计模块
-- ========================================

-- 成本中心表
CREATE TABLE IF NOT EXISTS cost_centers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    center_code VARCHAR(50) NOT NULL,
    center_name VARCHAR(200) NOT NULL,
    center_type VARCHAR(20), -- department, project, product
    parent_id UUID,
    manager VARCHAR(100),
    status VARCHAR(20) DEFAULT 'active',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, center_code)
);

-- 成本记录表
CREATE TABLE IF NOT EXISTS costs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    cost_date DATE NOT NULL,
    cost_center_id UUID NOT NULL,
    cost_element VARCHAR(100) NOT NULL,
    cost_type VARCHAR(50), -- material, labor, overhead
    amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(10) DEFAULT 'CNY',
    voucher_id UUID,
    remarks TEXT,
    created_by UUID,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 预算表
CREATE TABLE IF NOT EXISTS budgets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    budget_no VARCHAR(50) NOT NULL,
    budget_name VARCHAR(200) NOT NULL,
    budget_type VARCHAR(50), -- annual, quarterly, monthly
    fiscal_year INTEGER NOT NULL,
    department VARCHAR(100),
    total_amount DECIMAL(15,2) NOT NULL DEFAULT 0,
    used_amount DECIMAL(15,2) NOT NULL DEFAULT 0,
    available_amount DECIMAL(15,2) GENERATED ALWAYS AS (total_amount - used_amount) STORED,
    status VARCHAR(20) DEFAULT 'draft', -- draft, approved, active, closed
    approved_by UUID,
    approved_at TIMESTAMP,
    created_by UUID,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, budget_no)
);

-- 预算明细表
CREATE TABLE IF NOT EXISTS budget_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    budget_id UUID NOT NULL,
    account_id UUID NOT NULL,
    period VARCHAR(20) NOT NULL, -- 2024-01, 2024-Q1, etc.
    budget_amount DECIMAL(15,2) NOT NULL DEFAULT 0,
    used_amount DECIMAL(15,2) NOT NULL DEFAULT 0,
    available_amount DECIMAL(15,2) GENERATED ALWAYS AS (budget_amount - used_amount) STORED,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 资金管理表
CREATE TABLE IF NOT EXISTS funds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    fund_no VARCHAR(50) NOT NULL,
    fund_name VARCHAR(200) NOT NULL,
    fund_type VARCHAR(50), -- operating, investment, financing
    bank_account VARCHAR(100),
    bank_name VARCHAR(200),
    currency VARCHAR(10) DEFAULT 'CNY',
    opening_balance DECIMAL(15,2) DEFAULT 0,
    current_balance DECIMAL(15,2) DEFAULT 0,
    status VARCHAR(20) DEFAULT 'active',
    remarks TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, fund_no)
);

-- ========================================
-- 核心模块三：运营支持模块
-- ========================================

-- 银行账户表
CREATE TABLE IF NOT EXISTS bank_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    account_no VARCHAR(100) NOT NULL,
    account_name VARCHAR(200) NOT NULL,
    bank_name VARCHAR(200) NOT NULL,
    bank_code VARCHAR(50),
    branch_name VARCHAR(200),
    currency VARCHAR(10) DEFAULT 'CNY',
    account_type VARCHAR(50), -- checking, savings, etc.
    current_balance DECIMAL(15,2) DEFAULT 0,
    status VARCHAR(20) DEFAULT 'active',
    remarks TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, account_no)
);

-- 现金交易表
CREATE TABLE IF NOT EXISTS cash_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    transaction_no VARCHAR(50) NOT NULL,
    transaction_date DATE NOT NULL,
    transaction_type VARCHAR(20) NOT NULL, -- income, expense, transfer
    bank_account_id UUID,
    counterparty VARCHAR(200),
    description TEXT,
    amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(10) DEFAULT 'CNY',
    balance_after DECIMAL(15,2),
    voucher_id UUID,
    status VARCHAR(20) DEFAULT 'completed',
    created_by UUID,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, transaction_no)
);

-- 费用表
CREATE TABLE IF NOT EXISTS expenses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    expense_no VARCHAR(50) NOT NULL,
    expense_type VARCHAR(100) NOT NULL,
    applicant VARCHAR(100) NOT NULL,
    department VARCHAR(100),
    expense_date DATE NOT NULL,
    amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(10) DEFAULT 'CNY',
    budget_id UUID,
    status VARCHAR(20) DEFAULT 'pending', -- pending, approved, rejected, paid
    approved_by UUID,
    approved_at TIMESTAMP,
    paid_by UUID,
    paid_at TIMESTAMP,
    remarks TEXT,
    attachments TEXT[],
    created_by UUID,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, expense_no)
);

-- 发票管理表
CREATE TABLE IF NOT EXISTS invoices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    invoice_no VARCHAR(100) NOT NULL,
    invoice_type VARCHAR(20) NOT NULL, -- input, output
    invoice_code VARCHAR(50),
    invoice_date DATE NOT NULL,
    counterparty VARCHAR(200) NOT NULL,
    tax_number VARCHAR(50),
    amount_before_tax DECIMAL(15,2) NOT NULL,
    tax_rate DECIMAL(5,2) NOT NULL,
    tax_amount DECIMAL(15,2) NOT NULL,
    total_amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(10) DEFAULT 'CNY',
    status VARCHAR(20) DEFAULT 'valid', -- valid, invalid, cancelled
    certified BOOLEAN DEFAULT false,
    certified_at TIMESTAMP,
    remarks TEXT,
    created_by UUID,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, invoice_no)
);

-- ========================================
-- 核心模块四：财务报告与合规模块
-- ========================================

-- 财务报表表
CREATE TABLE IF NOT EXISTS reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    report_no VARCHAR(50) NOT NULL,
    report_name VARCHAR(200) NOT NULL,
    report_type VARCHAR(50) NOT NULL, -- balance_sheet, income_statement, cash_flow
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    currency VARCHAR(10) DEFAULT 'CNY',
    report_data JSONB,
    status VARCHAR(20) DEFAULT 'draft', -- draft, published
    generated_by UUID,
    generated_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, report_no)
);

-- 税务管理表
CREATE TABLE IF NOT EXISTS taxes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    tax_no VARCHAR(50) NOT NULL,
    tax_type VARCHAR(50) NOT NULL, -- vat, income_tax, surcharge
    tax_period VARCHAR(20) NOT NULL, -- 2024-01, 2024-Q1
    taxable_amount DECIMAL(15,2) NOT NULL,
    tax_rate DECIMAL(5,2) NOT NULL,
    tax_amount DECIMAL(15,2) NOT NULL,
    paid_amount DECIMAL(15,2) DEFAULT 0,
    outstanding_amount DECIMAL(15,2) GENERATED ALWAYS AS (tax_amount - paid_amount) STORED,
    due_date DATE,
    payment_date DATE,
    status VARCHAR(20) DEFAULT 'unpaid', -- unpaid, partial, paid
    remarks TEXT,
    created_by UUID,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, tax_no)
);

-- 合并报表表
CREATE TABLE IF NOT EXISTS consolidations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    consolidation_no VARCHAR(50) NOT NULL,
    consolidation_name VARCHAR(200) NOT NULL,
    fiscal_year INTEGER NOT NULL,
    period VARCHAR(20) NOT NULL, -- annual, quarterly
    consolidation_type VARCHAR(50), -- full_consolidation, equity_method
    parent_org_id UUID NOT NULL,
    subsidiary_org_ids UUID[] NOT NULL,
    elimination_entries JSONB,
    status VARCHAR(20) DEFAULT 'draft', -- draft, completed
    created_by UUID,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, consolidation_no)
);

-- ========================================
-- 创建索引
-- ========================================

CREATE INDEX IF NOT EXISTS idx_accounts_tenant_org ON accounts(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_accounts_code ON accounts(account_code);
CREATE INDEX IF NOT EXISTS idx_vouchers_tenant_org ON vouchers(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_vouchers_date ON vouchers(voucher_date);
CREATE INDEX IF NOT EXISTS idx_voucher_items_voucher ON voucher_items(voucher_id);
CREATE INDEX IF NOT EXISTS idx_receivables_tenant_org ON receivables(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_receivables_customer ON receivables(customer_id);
CREATE INDEX IF NOT EXISTS idx_payables_tenant_org ON payables(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_payables_supplier ON payables(supplier_id);
CREATE INDEX IF NOT EXISTS idx_assets_tenant_org ON assets(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_asset_depreciations_asset ON asset_depreciations(asset_id);
CREATE INDEX IF NOT EXISTS idx_costs_tenant_org ON costs(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_costs_date ON costs(cost_date);
CREATE INDEX IF NOT EXISTS idx_budgets_tenant_org ON budgets(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_budget_items_budget ON budget_items(budget_id);
CREATE INDEX IF NOT EXISTS idx_funds_tenant_org ON funds(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_bank_accounts_tenant_org ON bank_accounts(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_cash_transactions_tenant_org ON cash_transactions(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_cash_transactions_date ON cash_transactions(transaction_date);
CREATE INDEX IF NOT EXISTS idx_expenses_tenant_org ON expenses(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_invoices_tenant_org ON invoices(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_reports_tenant_org ON reports(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_taxes_tenant_org ON taxes(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_consolidations_tenant_org ON consolidations(tenant_id, org_id);
