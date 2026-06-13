import { BrowserRouter, Routes, Route } from "react-router-dom";
import { AuthProvider } from "./auth/AuthContext";
import Layout from "./components/Layout";
import ProtectedRoute from "./components/ProtectedRoute";
import LoginPage from "./auth/LoginPage";
import Dashboard from "./pages/Dashboard";
import CustomerList from "./pages/CustomerList";
import CustomerForm from "./pages/CustomerForm";
import ProductList from "./pages/ProductList";
import ProductForm from "./pages/ProductForm";
import SupplierList from "./pages/SupplierList";
import SupplierForm from "./pages/SupplierForm";
import OrderList from "./pages/OrderList";
import OrderDetail from "./pages/OrderDetail";
import PurchaseOrderList from "./pages/PurchaseOrderList";
import PurchaseOrderDetail from "./pages/PurchaseOrderDetail";
import InvoiceList from "./pages/InvoiceList";
import InvoiceForm from "./pages/InvoiceForm";
import SalesReport from "./pages/SalesReport";
import InventoryReport from "./pages/InventoryReport";
import AgingReport from "./pages/AgingReport";

export default function App() {
  return (
    <BrowserRouter>
      <AuthProvider>
        <Routes>
          <Route path="/login" element={<LoginPage />} />
          <Route element={<ProtectedRoute />}>
            <Route element={<Layout />}>
              <Route index element={<Dashboard />} />
              <Route path="customers" element={<CustomerList />} />
              <Route path="customers/new" element={<CustomerForm />} />
              <Route path="customers/:id/edit" element={<CustomerForm />} />
              <Route path="products" element={<ProductList />} />
              <Route path="products/new" element={<ProductForm />} />
              <Route path="products/:id/edit" element={<ProductForm />} />
              <Route path="suppliers" element={<SupplierList />} />
              <Route path="suppliers/new" element={<SupplierForm />} />
              <Route path="suppliers/:id/edit" element={<SupplierForm />} />
              <Route path="orders" element={<OrderList />} />
              <Route path="orders/:id" element={<OrderDetail />} />
              <Route path="purchase-orders" element={<PurchaseOrderList />} />
              <Route path="purchase-orders/:id" element={<PurchaseOrderDetail />} />
              <Route path="invoices" element={<InvoiceList />} />
              <Route path="invoices/new" element={<InvoiceForm />} />
              <Route path="reports/sales" element={<SalesReport />} />
              <Route path="reports/inventory" element={<InventoryReport />} />
              <Route path="reports/aging" element={<AgingReport />} />
            </Route>
          </Route>
        </Routes>
      </AuthProvider>
    </BrowserRouter>
  );
}
