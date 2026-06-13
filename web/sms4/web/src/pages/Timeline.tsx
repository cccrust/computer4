import { useState, useEffect, useCallback } from "react";
import { api } from "../api/client";
import type { PostWithUser, User } from "../types";
import PostCard from "../components/PostCard";
import PostForm from "../components/PostForm";

export default function Timeline() {
  const [posts, setPosts] = useState<PostWithUser[]>([]);
  const [users, setUsers] = useState<User[]>([]);
  const [currentUserId, setCurrentUserId] = useState<number | null>(null);
  const [loading, setLoading] = useState(true);
  const [likedPosts, setLikedPosts] = useState<Set<number>>(new Set());

  const fetchPosts = useCallback(async () => {
    try {
      const data = await api.posts.list();
      setPosts(data);
    } catch {
      // ignore
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    const init = async () => {
      try {
        const userList = await api.users.list();
        setUsers(userList);
        if (userList.length > 0) {
          setCurrentUserId(userList[0].id);
        }
      } catch {
        // ignore
      }
      await fetchPosts();
    };
    init();
  }, [fetchPosts]);

  const handlePost = async (content: string) => {
    if (currentUserId == null) return;
    const newPost = await api.posts.create({ user_id: currentUserId, content });
    setPosts((prev) => [newPost, ...prev]);
  };

  const handleLike = async (postId: number) => {
    if (currentUserId == null) return;
    if (likedPosts.has(postId)) {
      await api.likes.remove(currentUserId, postId);
      setLikedPosts((prev) => { const n = new Set(prev); n.delete(postId); return n; });
      setPosts((prev) => prev.map((p) => p.id === postId ? { ...p, likes_count: p.likes_count - 1 } : p));
    } else {
      await api.likes.add(currentUserId, postId);
      setLikedPosts((prev) => { const n = new Set(prev); n.add(postId); return n; });
      setPosts((prev) => prev.map((p) => p.id === postId ? { ...p, likes_count: p.likes_count + 1 } : p));
    }
  };

  const handleReply = (postId: number) => {
    window.location.href = `/posts/${postId}`;
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center py-20 text-gray-500">
        載入中...
      </div>
    );
  }

  return (
    <div>
      <PostForm onSubmit={handlePost} />
      {currentUserId != null && (
        <div className="px-4 py-2 text-xs text-gray-500 border-b border-gray-800">
          以 @{users.find((u) => u.id === currentUserId)?.username ?? "?"} 發布
        </div>
      )}
      {users.length > 1 && (
        <div className="flex gap-2 px-4 py-2 border-b border-gray-800 overflow-x-auto">
          {users.map((u) => (
            <button
              key={u.id}
              onClick={() => setCurrentUserId(u.id)}
              className={`shrink-0 text-xs px-3 py-1 rounded-full border transition ${
                currentUserId === u.id
                  ? "bg-white text-black border-white"
                  : "bg-transparent text-gray-400 border-gray-700 hover:border-gray-500"
              }`}
            >
              @{u.username}
            </button>
          ))}
        </div>
      )}
      {posts.length === 0 ? (
        <div className="text-center py-20 text-gray-500">
          <p className="text-lg">尚無貼文</p>
          <p className="text-sm mt-1">成為第一個發布貼文的人！</p>
        </div>
      ) : (
        posts.map((post) => (
          <PostCard
            key={post.id}
            post={post}
            showThread
            onLike={() => handleLike(post.id)}
            onReply={() => handleReply(post.id)}
            liked={likedPosts.has(post.id)}
          />
        ))
      )}
    </div>
  );
}
