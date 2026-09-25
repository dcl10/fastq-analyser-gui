import type { FastaSeqResult } from "@/types/results";

export function FastaResultPanel({ result }: { result: FastaSeqResult }) {
  return (
    <dl className="grid grid-cols-[max-content_1fr] gap-x-2 gap-y-1 text-sm">
      <dt className="font-semibold">Description:</dt>
      <dd>{result.desc}</dd>
      <dt className="font-semibold">Sequence length:</dt>
      <dd>{result.seq_len} bases</dd>
      <dt className="font-semibold">GC %:</dt>
      <dd>{result.gc * 100}%</dd>
      <dt className="font-semibold">No.# ORFs:</dt>
      <dd>{result.n_orfs}</dd>
    </dl>
  );
}
