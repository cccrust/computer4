import { useNavigate } from "react-router-dom";
import type { PostWithUser } from "../types";

interface Props {
  post: PostWithUser;
  showThread?: boolean;
  onLike?: () => void;
  onReply?: () => void;
  liked?: boolean;
}

export default function PostCard({ post, showThread, onLike, onReply, liked }: Props) {
  const navigate = useNavigate();

  return (
    <div className="border-b border-gray-800 px-4 py-3 hover:bg-gray-900/50 transition">
      <div className="flex gap-3">
        <div
          className="w-10 h-10 rounded-full bg-gray-700 flex items-center justify-center text-sm font-bold shrink-0 cursor-pointer"
          onClick={() => navigate(`/users/${post.user_id}`)}
        >
          {post.display_name[0]}
        </div>
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 text-sm">
            <span
              className="font-bold text-white cursor-pointer hover:underline"
              onClick={() => navigate(`/users/${post.user_id}`)}
            >
              {post.display_name}
            </span>
            <span className="text-gray-500">@{post.username}</span>
            <span className="text-gray-500">·</span>
            <span className="text-gray-500 text-xs">
              {new Date(post.created_at).toLocaleDateString("zh-TW", { month: "short", day: "numeric" })}
            </span>
          </div>
          <div
            className="mt-1 text-[15px] leading-relaxed text-white cursor-pointer"
            onClick={() => navigate(`/posts/${post.id}`)}
          >
            {post.content}
          </div>
          <div className="flex items-center gap-6 mt-3 text-gray-500">
            <button
              onClick={(e) => { e.stopPropagation(); onReply?.(); }}
              className="flex items-center gap-1 text-sm hover:text-blue-500 transition"
            >
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
              </svg>
              {post.replies_count > 0 && <span>{post.replies_count}</span>}
            </button>
            <button
              onClick={(e) => { e.stopPropagation(); onLike?.(); }}
              className={`flex items-center gap-1 text-sm hover:text-red-500 transition ${liked ? "text-red-500" : ""}`}
            >
              <svg className="w-4 h-4" fill={liked ? "currentColor" : "none"} stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
              </svg>
              {post.likes_count > 0 && <span>{post.likes_count}</span>}
            </button>
          </div>
          {showThread && post.replies_count > 0 && (
            <button
              onClick={() => navigate(`/posts/${post.id}`)}
              className="mt-1 text-sm text-blue-500 hover:text-blue-400"
            >
              查看回覆 ({post.replies_count})
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
