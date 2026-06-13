export interface User {
  id: number;
  username: string;
  display_name: string;
  role: string;
}

export interface Customer {
  id: number;
  name: string;
  email: string | null;
  phone: string | null;
  address: string | null;
  created_at: string;
  updated_at: string;
}

export interface Product {
  id: number;
  name: string;
  sku: string;
  price: number;
  stock: number;
  description: string | null;
  created_at: string;
  updated_at: string;
}

export interface Supplier {
  id: number;
  name: string;
  contact_person: string | null;
  email: string | null;
  phone: string | null;
  address: string | null;
  created_at: string;
  updated_at: string;
}

export interface Order {
  id: number;
  customer_id: number;
  order_date: string;
  status: string;
  total_amount: number;
  notes: string | null;
  created_at: string;
  updated_at: string;
}

export interface OrderItem {
  id: number;
  order_id: number;
  product_id: number;
  quantity: number;
  unit_price: number;
}

export interface PurchaseOrder {
  id: number;
  supplier_id: number;
  order_date: string;
  status: string;
  total_amount: number;
  notes: string | null;
  created_at: string;
  updated_at: string;
}

export interface POItem {
  id: number;
  po_id: number;
  product_id: number;
  quantity: number;
  unit_price: number;
}

export interface Invoice {
  id: number;
  invoice_number: string;
  order_id: number | null;
  customer_id: number;
  invoice_date: string;
  due_date: string;
  status: string;
  amount: number;
  notes: string | null;
  created_at: string;
  updated_at: string;
}

export interface SalesRow {
  product_id: number;
  product_name: string;
  total_qty: number;
  total_amount: number;
}

export interface DashboardData {
  customer_count: number;
  product_count: number;
  low_stock_count: number;
  pending_orders: number;
  pending_pos: number;
  overdue_invoices_total: number;
  recent_orders: Order[];
}

export interface LoginResponse {
  token: string;
  user: User;
}
