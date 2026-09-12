import { DnaMotif } from "@/components/brand/dna-motif";

interface LoadingIndicatorProps {
  message: string;
}

export function LoadingIndicator({ message }: LoadingIndicatorProps) {
  return (
    <div className="flex items-center gap-3 py-4">
      <DnaMotif variant="loader" size={24} />
      <p>{message}</p>
    </div>
  );
}
