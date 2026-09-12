import { AppShell } from "@/components/app-shell";
import { FastqAnalyserApp } from "@/components/fastq-analyser-app";

export default function Home() {
  return (
    <AppShell>
      <FastqAnalyserApp />
    </AppShell>
  );
}
