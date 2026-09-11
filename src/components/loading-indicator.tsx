import { Loader2 } from "lucide-react";

interface LoadingIndicatorProps {
  message: string;
}

export function LoadingIndicator({ message }: LoadingIndicatorProps) {
  return (
    <div className="flex items-center gap-3 py-4">
      <Loader2 className="size-6 animate-spin text-primary" />
      <p>{message}</p>
    </div>
  );
}
