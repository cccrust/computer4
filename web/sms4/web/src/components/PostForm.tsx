import { useState } from "react";

interface Props {
  placeholder?: string;
  buttonLabel?: string;
  onSubmit: (content: string) => Promise<void>;
}

export default function PostForm({ placeholder = "有什麼想法？", buttonLabel = "發布", onSubmit }: Props) {
  const [content, setContent] = useState("");
  const [submitting, setSubmitting] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!content.trim() || submitting) return;
    setSubmitting(true);
    try {
      await onSubmit(content.trim());
      setContent("");
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="border-b border-gray-800 px-4 py-3">
      <div className="flex gap-3">
        <div className="flex-1">
          <textarea
            value={content}
            onChange={(e) => setContent(e.target.value)}
            placeholder={placeholder}
            rows={2}
            maxLength={500}
            className="w-full bg-transparent text-white text-[15px] placeholder-gray-500 resize-none outline-none"
          />
          <div className="flex items-center justify-between mt-2">
            <span className="text-xs text-gray-500">{content.length}/500</span>
            <button
              type="submit"
              disabled={!content.trim() || submitting}
              className="bg-blue-500 text-white px-4 py-1.5 rounded-full text-sm font-bold hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed transition"
            >
              {submitting ? "發布中..." : buttonLabel}
            </button>
          </div>
        </div>
      </div>
    </form>
  );
}
