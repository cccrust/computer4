import { useNavigate } from "react-router-dom";
import type { User } from "../types";

interface Props {
  user: User;
}

export default function UserCard({ user }: Props) {
  const navigate = useNavigate();

  return (
    <div
      className="border-b border-gray-800 px-4 py-3 hover:bg-gray-900/50 transition cursor-pointer"
      onClick={() => navigate(`/users/${user.id}`)}
    >
      <div className="flex items-center gap-3">
        <div className="w-12 h-12 rounded-full bg-gray-700 flex items-center justify-center text-lg font-bold shrink-0">
          {user.display_name[0]}
        </div>
        <div className="flex-1 min-w-0">
          <div className="font-bold text-white">{user.display_name}</div>
          <div className="text-sm text-gray-500">@{user.username}</div>
          {user.bio && (
            <div className="text-sm text-gray-400 mt-0.5 line-clamp-1">{user.bio}</div>
          )}
        </div>
        <svg className="w-5 h-5 text-gray-500 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
        </svg>
      </div>
    </div>
  );
}
