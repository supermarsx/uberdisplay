"use client";

import { useEffect } from "react";

export type ToastState =
  | {
      message: string;
      type?: "info" | "success" | "error";
    }
  | null;

type ToastProps = {
  toast: ToastState;
  onClear: () => void;
};

export default function Toast({ toast, onClear }: ToastProps) {
  useEffect(() => {
    if (!toast) {
      return;
    }
    const timeout = toast.type === "error" ? 6500 : 3800;
    const timer = setTimeout(() => onClear(), timeout);
    return () => clearTimeout(timer);
  }, [toast, onClear]);

  if (!toast) {
    return null;
  }

  const tone = toast.type ?? "info";
  return (
    <div className="toast-stack" role="status" aria-live="polite">
      <button className={`toast toast--${tone}`} type="button" onClick={onClear}>
        <span className="toast-title">
          {tone === "error" ? "Warning" : tone === "success" ? "Updated" : "Notice"}
        </span>
        <span className="toast-body">{toast.message}</span>
      </button>
    </div>
  );
}
