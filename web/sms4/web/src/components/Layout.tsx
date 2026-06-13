import { NavLink, Outlet } from "react-router-dom";

const navItems = [
  { label: "首頁", to: "/", icon: "🏠" },
  { label: "配對", to: "/search", icon: "🔍" },
  { label: "私訊", to: "/messages", icon: "✉️" },
  { label: "使用者", to: "/users", icon: "👤" },
];

export default function Layout() {
  return (
    <div className="h-full flex flex-col max-w-lg mx-auto bg-black">
      <header className="sticky top-0 z-10 bg-black/80 backdrop-blur border-b border-gray-800 px-4 py-3">
        <h1 className="text-xl font-bold text-white text-center">SMS4</h1>
      </header>

      <main className="flex-1 overflow-y-auto">
        <Outlet />
      </main>

      <nav className="sticky bottom-0 bg-black border-t border-gray-800">
        <div className="flex justify-around py-2">
          {navItems.map((item) => (
            <NavLink
              key={item.to}
              to={item.to}
              end={item.to === "/"}
              className={({ isActive }) =>
                `flex flex-col items-center px-6 py-1 text-xs transition ${
                  isActive ? "text-white" : "text-gray-500"
                }`
              }
            >
              <span className="text-xl mb-0.5">{item.icon}</span>
              <span>{item.label}</span>
            </NavLink>
          ))}
        </div>
      </nav>
    </div>
  );
}
