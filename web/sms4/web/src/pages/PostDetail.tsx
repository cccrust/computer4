import { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { api } from "../api/client";
import type { PostDetail as PostDetailType } from "../types";
import PostCard from "../components/PostCard";
import PostForm from "../components/PostForm";

export default function PostDetail() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [data, setData] = useState<PostDetailType | null>(null);
  const [loading, setLoading] = useState(true);
  const [likedPosts, setLikedPosts] = useState<Set<number>>(new Set());
  const [currentUserId, setCurrentUserId] = useState<number | null>(null);

  const fetchPost = async () => {
    if (!id) return;
    try {
      const postData = await api.posts.get(parseInt(id));
      setData(postData);
    } catch {
      // ignore
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    const init = async () => {
      try {
        const users = await api.users.list();
        if (users.length > 0) setCurrentUserId(users[0].id);
      } catch {
        // ignore
      }
      await fetchPost();
    };
    init();
  }, [id]);

  const handleReply = async (content: string) => {
    if (!id || currentUserId == null) return;
    await api.posts.reply(parseInt(id), { user_id: currentUserId, content });
    await fetchPost();
  };

  const handleLike = async (postId: number) => {
    if (currentUserId == null) return;
    if (likedPosts.has(postId)) {
      await api.likes.remove(currentUserId, postId);
      setLikedPosts((prev) => { const n = new Set(prev); n.delete(postId); return n; });
    } else {
      await api.likes.add(currentUserId, postId);
      setLikedPosts((prev) => { const n = new Set(prev); n.add(postId); return n; });
    }
    await fetchPost();
  };

  if (loading) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  if (!data) {
    return (
      <div className="text-center py-20 text-gray-500">
        <p className="text-lg">貼文不存在</p>
        <button
          onClick={() => navigate("/")}
          className="mt-4 text-blue-500 text-sm hover:underline"
        >
          返回首頁
        </button>
      </div>
    );
  }

  return (
    <div>
      <div className="px-4 py-3 border-b border-gray-800">
        <button
          onClick={() => navigate(-1)}
          className="text-white inline-flex items-center gap-1 text-sm"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
          </svg>
          返回
        </button>
      </div>

      <PostCard
        post={data.post}
        onLike={() => handleLike(data.post.id)}
        onReply={() => document.querySelector<HTMLTextAreaElement>("textarea")?.focus()}
        liked={likedPosts.has(data.post.id)}
      />

      <div className="border-b border-gray-800">
        <PostForm
          placeholder="回覆這則貼文..."
          buttonLabel="回覆"
          onSubmit={handleReply}
        />
      </div>

      {data.replies.length === 0 ? (
        <div className="text-center py-10 text-gray-500 text-sm">
          尚無回覆，來回覆這則貼文吧！
        </div>
      ) : (
        data.replies.map((reply) => (
          <PostCard
            key={reply.id}
            post={reply}
            onLike={() => handleLike(reply.id)}
            onReply={() => {}}
            liked={likedPosts.has(reply.id)}
          />
        ))
      )}
    </div>
  );
}
