import { NavLink, Outlet } from "react-router-dom";
import { useAuth } from "../auth/AuthContext";

const navItems = [
  { label: "儀表板", to: "/" },
  { label: "客戶", to: "/customers" },
  { label: "產品", to: "/products" },
  { label: "供應商", to: "/suppliers" },
  { label: "訂單", to: "/orders" },
  { label: "採購單", to: "/purchase-orders" },
  { label: "發票", to: "/invoices" },
];

const reportSubItems = [
  { label: "銷售報表", to: "/reports/sales" },
  { label: "庫存報表", to: "/reports/inventory" },
  { label: "帳齡報表", to: "/reports/aging" },
];

export default function Layout() {
  const { user, logout } = useAuth();

  return (
    <div className="flex h-screen bg-gray-100">
      <aside className="w-64 bg-gray-900 text-white flex flex-col shrink-0">
        <div className="h-16 flex items-center px-6 border-b border-gray-700">
          <h1 className="text-xl font-bold">ERP4</h1>
        </div>
        <nav className="flex-1 overflow-y-auto py-4 px-3 space-y-1">
          {navItems.map((item) => (
            <NavLink
              key={item.to}
              to={item.to}
              end={item.to === "/"}
              className={({ isActive }) =>
                `block px-3 py-2 rounded text-sm transition ${
                  isActive
                    ? "bg-gray-700 text-white"
                    : "text-gray-300 hover:bg-gray-700 hover:text-white"
                }`
              }
            >
              {item.label}
            </NavLink>
          ))}
          <div className="pt-2">
            <div className="px-3 py-2 text-xs text-gray-400 uppercase tracking-wider">
              報表
            </div>
            {reportSubItems.map((item) => (
              <NavLink
                key={item.to}
                to={item.to}
                className={({ isActive }) =>
                  `block px-3 py-2 rounded text-sm transition ${
                    isActive
                      ? "bg-gray-700 text-white"
                      : "text-gray-300 hover:bg-gray-700 hover:text-white"
                  }`
                }
              >
                {item.label}
              </NavLink>
            ))}
          </div>
        </nav>
      </aside>

      <div className="flex-1 flex flex-col min-w-0">
        <header className="h-16 bg-white shadow flex items-center justify-end px-6 gap-4 shrink-0">
          <span className="text-sm text-gray-700">{user?.display_name}</span>
          <button
            onClick={logout}
            className="text-sm text-red-600 hover:text-red-800"
          >
            登出
          </button>
        </header>

        <main className="flex-1 overflow-y-auto p-6">
          <Outlet />
        </main>
      </div>
    </div>
  );
}
