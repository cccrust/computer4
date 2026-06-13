import { useState, useEffect } from "react";
import { api } from "../api/client";
import type { User } from "../types";
import UserCard from "../components/UserCard";

export default function UserList() {
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(true);
  const [search, setSearch] = useState("");

  useEffect(() => {
    const fetchUsers = async () => {
      setLoading(true);
      try {
        const params: Record<string, string> = {};
        if (search) params.search = search;
        const data = await api.users.list(params);
        setUsers(data);
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    };
    fetchUsers();
  }, [search]);

  return (
    <div>
      <div className="sticky top-0 bg-black/80 backdrop-blur px-4 py-3 border-b border-gray-800">
        <input
          type="text"
          placeholder="搜尋使用者..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="w-full bg-gray-900 text-white rounded-full px-4 py-2 text-sm placeholder-gray-500 outline-none focus:ring-1 focus:ring-gray-600"
        />
      </div>
      {loading ? (
        <div className="text-center py-20 text-gray-500">載入中...</div>
      ) : users.length === 0 ? (
        <div className="text-center py-20 text-gray-500">
          <p className="text-lg">查無使用者</p>
        </div>
      ) : (
        users.map((user) => (
          <UserCard key={user.id} user={user} />
        ))
      )}
    </div>
  );
}
