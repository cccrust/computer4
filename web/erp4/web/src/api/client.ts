const BASE = "/api";

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const token = localStorage.getItem("token");
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
    ...(options?.headers as Record<string, string>),
  };
  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  }
  const res = await fetch(`${BASE}${path}`, { ...options, headers });
  if (res.status === 401) {
    localStorage.removeItem("token");
    localStorage.removeItem("user");
    window.location.href = "/login";
    throw new Error("未授權");
  }
  if (!res.ok) {
    const body = await res.json().catch(() => ({ error: res.statusText }));
    throw new Error(body.error || "請求失敗");
  }
  return res.json();
}

export const api = {
  login: (username: string, password: string) =>
    request<import("../types").LoginResponse>("/login", {
      method: "POST",
      body: JSON.stringify({ username, password }),
    }),
  me: () => request<import("../types").User>("/me"),
  dashboard: () => request<import("../types").DashboardData>("/dashboard"),
  customers: {
    list: (params?: Record<string, string>) =>
      request<import("../types").Customer[]>(
        `/customers?${new URLSearchParams(params)}`,
      ),
    get: (id: number) => request<import("../types").Customer>(`/customers/${id}`),
    create: (data: Record<string, unknown>) =>
      request<import("../types").Customer>("/customers", {
        method: "POST",
        body: JSON.stringify(data),
      }),
    update: (id: number, data: Record<string, unknown>) =>
      request<import("../types").Customer>(`/customers/${id}`, {
        method: "PUT",
        body: JSON.stringify(data),
      }),
    delete: (id: number) => request<{ deleted: boolean }>(`/customers/${id}`, { method: "DELETE" }),
  },
  products: {
    list: (params?: Record<string, string>) =>
      request<import("../types").Product[]>(
        `/products?${new URLSearchParams(params)}`,
      ),
    get: (id: number) => request<import("../types").Product>(`/products/${id}`),
    create: (data: Record<string, unknown>) =>
      request<import("../types").Product>("/products", {
        method: "POST",
        body: JSON.stringify(data),
      }),
    update: (id: number, data: Record<string, unknown>) =>
      request<import("../types").Product>(`/products/${id}`, {
        method: "PUT",
        body: JSON.stringify(data),
      }),
    delete: (id: number) => request<{ deleted: boolean }>(`/products/${id}`, { method: "DELETE" }),
  },
  suppliers: {
    list: (params?: Record<string, string>) =>
      request<import("../types").Supplier[]>(
        `/suppliers?${new URLSearchParams(params)}`,
      ),
    get: (id: number) => request<import("../types").Supplier>(`/suppliers/${id}`),
    create: (data: Record<string, unknown>) =>
      request<import("../types").Supplier>("/suppliers", {
        method: "POST",
        body: JSON.stringify(data),
      }),
    update: (id: number, data: Record<string, unknown>) =>
      request<import("../types").Supplier>(`/suppliers/${id}`, {
        method: "PUT",
        body: JSON.stringify(data),
      }),
    delete: (id: number) => request<{ deleted: boolean }>(`/suppliers/${id}`, { method: "DELETE" }),
  },
  orders: {
    list: (params?: Record<string, string>) =>
      request<import("../types").Order[]>(`/orders?${new URLSearchParams(params)}`),
    get: (id: number) => request<import("../types").Order>(`/orders/${id}`),
    create: (data: Record<string, unknown>) =>
      request<import("../types").Order>("/orders", {
        method: "POST",
        body: JSON.stringify(data),
      }),
    updateStatus: (id: number, status: string) =>
      request<import("../types").Order>(`/orders/${id}/status`, {
        method: "POST",
        body: JSON.stringify({ status }),
      }),
    delete: (id: number) => request<{ deleted: boolean }>(`/orders/${id}`, { method: "DELETE" }),
    listItems: (id: number) =>
      request<import("../types").OrderItem[]>(`/orders/${id}/items`),
    addItem: (id: number, data: Record<string, unknown>) =>
      request<import("../types").OrderItem>(`/orders/${id}/items`, {
        method: "POST",
        body: JSON.stringify(data),
      }),
  },
  purchaseOrders: {
    list: (params?: Record<string, string>) =>
      request<import("../types").PurchaseOrder[]>(
        `/purchase-orders?${new URLSearchParams(params)}`,
      ),
    get: (id: number) =>
      request<import("../types").PurchaseOrder>(`/purchase-orders/${id}`),
    create: (data: Record<string, unknown>) =>
      request<import("../types").PurchaseOrder>("/purchase-orders", {
        method: "POST",
        body: JSON.stringify(data),
      }),
    updateStatus: (id: number, status: string) =>
      request<import("../types").PurchaseOrder>(`/purchase-orders/${id}/status`, {
        method: "POST",
        body: JSON.stringify({ status }),
      }),
    delete: (id: number) =>
      request<{ deleted: boolean }>(`/purchase-orders/${id}`, { method: "DELETE" }),
    listItems: (id: number) =>
      request<import("../types").POItem[]>(`/purchase-orders/${id}/items`),
    addItem: (id: number, data: Record<string, unknown>) =>
      request<import("../types").POItem>(`/purchase-orders/${id}/items`, {
        method: "POST",
        body: JSON.stringify(data),
      }),
  },
  invoices: {
    list: (params?: Record<string, string>) =>
      request<import("../types").Invoice[]>(
        `/invoices?${new URLSearchParams(params)}`,
      ),
    get: (id: number) => request<import("../types").Invoice>(`/invoices/${id}`),
    create: (data: Record<string, unknown>) =>
      request<import("../types").Invoice>("/invoices", {
        method: "POST",
        body: JSON.stringify(data),
      }),
    updateStatus: (id: number, status: string) =>
      request<import("../types").Invoice>(`/invoices/${id}/status`, {
        method: "POST",
        body: JSON.stringify({ status }),
      }),
    delete: (id: number) =>
      request<{ deleted: boolean }>(`/invoices/${id}`, { method: "DELETE" }),
  },
  reports: {
    sales: (params?: Record<string, string>) =>
      request<import("../types").SalesRow[]>(
        `/reports/sales?${new URLSearchParams(params)}`,
      ),
    inventory: () => request<import("../types").Product[]>("/reports/inventory"),
    aging: () => request<import("../types").Invoice[]>("/reports/aging"),
  },
};
