import { useState, useEffect, useRef } from "react";
import { useParams, useSearchParams, useNavigate } from "react-router-dom";
import { api } from "../api/client";
import type { MessageWithUser } from "../types";

export default function Conversation() {
  const { otherId } = useParams<{ otherId: string }>();
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const [msgs, setMsgs] = useState<MessageWithUser[]>([]);
  const [otherUser, setOtherUser] = useState<{ id: number; username: string; display_name: string } | null>(null);
  const [content, setContent] = useState("");
  const bottomRef = useRef<HTMLDivElement>(null);

  const userId = parseInt(searchParams.get("uid") || "0");

  useEffect(() => {
    if (!otherId || !userId) return;
    const fetch = async () => {
      try {
        const [msgsData, users] = await Promise.all([
          api.messages.messages(userId, parseInt(otherId)),
          api.users.list(),
        ]);
        setMsgs(msgsData);
        const other = users.find((u) => u.id === parseInt(otherId));
        if (other) setOtherUser(other);
      } catch {
        // ignore
      }
    };
    fetch();
    const interval = setInterval(fetch, 3000);
    return () => clearInterval(interval);
  }, [otherId, userId]);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [msgs]);

  const handleSend = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!content.trim() || !otherId || !userId) return;
    await api.messages.send(userId, parseInt(otherId), content.trim());
    setContent("");
    const msgsData = await api.messages.messages(userId, parseInt(otherId));
    setMsgs(msgsData);
  };

  if (!otherId || !userId) {
    return <div className="text-center py-20 text-gray-500">參數錯誤</div>;
  }

  return (
    <div className="flex flex-col h-full">
      <div className="px-4 py-3 border-b border-gray-800 flex items-center gap-3">
        <button
          onClick={() => navigate("/messages")}
          className="text-white"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
          </svg>
        </button>
        {otherUser && (
          <div className="flex items-center gap-2">
            <div className="w-8 h-8 rounded-full bg-gray-700 flex items-center justify-center text-sm font-bold">
              {otherUser.display_name[0]}
            </div>
            <div>
              <div className="font-bold text-white text-sm">{otherUser.display_name}</div>
              <div className="text-xs text-gray-500">@{otherUser.username}</div>
            </div>
          </div>
        )}
      </div>

      <div className="flex-1 overflow-y-auto px-4 py-4 space-y-3">
        {msgs.length === 0 ? (
          <div className="text-center py-10 text-gray-500 text-sm">尚無訊息，發送第一條訊息吧！</div>
        ) : (
          msgs.map((m) => {
            const isMine = m.sender_id === userId;
            return (
              <div key={m.id} className={`flex ${isMine ? "justify-end" : "justify-start"}`}>
                <div
                  className={`max-w-[75%] px-3 py-2 rounded-2xl text-sm ${
                    isMine
                      ? "bg-blue-500 text-white rounded-br-md"
                      : "bg-gray-800 text-white rounded-bl-md"
                  }`}
                >
                  <div>{m.content}</div>
                  <div className={`text-xs mt-1 ${isMine ? "text-blue-200" : "text-gray-500"}`}>
                    {new Date(m.created_at).toLocaleTimeString("zh-TW", { hour: "2-digit", minute: "2-digit" })}
                  </div>
                </div>
              </div>
            );
          })
        )}
        <div ref={bottomRef} />
      </div>

      <form onSubmit={handleSend} className="border-t border-gray-800 px-4 py-3 flex gap-2">
        <input
          type="text"
          value={content}
          onChange={(e) => setContent(e.target.value)}
          placeholder="輸入訊息..."
          maxLength={500}
          className="flex-1 bg-gray-900 text-white rounded-full px-4 py-2 text-sm placeholder-gray-500 outline-none focus:ring-1 focus:ring-gray-600"
        />
        <button
          type="submit"
          disabled={!content.trim()}
          className="bg-blue-500 text-white rounded-full px-4 py-2 text-sm font-bold hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed transition"
        >
          送出
        </button>
      </form>
    </div>
  );
}
