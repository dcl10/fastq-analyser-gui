"use client";

import { useRef, useState, type ChangeEvent } from "react";
import { useRouter } from "next/navigation";
import { open } from "@tauri-apps/plugin-dialog";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { Button } from "@/components/ui/button";
import { DnaMotif } from "@/components/brand/dna-motif";
import { FileInput } from "@/components/file-input";
import { FormatToggle } from "@/components/format-toggle";
import { LoadingIndicator } from "@/components/loading-indicator";
import { TextInput } from "@/components/text-input";
import { Toolbar } from "@/components/toolbar";
import { analyseFileSequences, analyseTextSequences } from "@/lib/analysis";
import { saveRun } from "@/lib/runs";
import type { SeqFormat } from "@/types/results";

export function FastqAnalyserApp() {
  const router = useRouter();
  const textSequences = useRef("");
  const fileSequences = useRef("");
  const [filePath, setFilePath] = useState("");
  const [seqFormat, setSeqFormat] = useState<SeqFormat>("fastq");
  const [isRunning, setIsRunning] = useState(false);
  const [error, setError] = useState("");

  // Change the text sequences in state
  const handleTextInput = (event: ChangeEvent<HTMLTextAreaElement>) => {
    textSequences.current = event.target.value;
  };

  // Change the file sequences in state
  const handleFileInput = async () => {
    const extensions =
      seqFormat === "fastq" ? ["fq", "fastq"] : ["fa", "fna", "fasta"];
    const selected = await open({
      directory: false,
      multiple: false,
      filters: [
        {
          name: "Sequence files",
          extensions,
        },
      ],
    });

    const fileInputEl = document.getElementById(
      "file-input",
    ) as HTMLInputElement | null;
    if (fileInputEl) fileInputEl.value = selected ?? "";
    fileSequences.current = selected ?? "";
    setFilePath(selected ?? "");
  };

  // Change the sequence format
  const handleFormatSwitch = (checked: boolean) => {
    setSeqFormat(checked ? "fastq" : "fasta");
  };

  // Clear the input fields and reset the state
  const clearInputs = () => {
    const textInputEl = document.getElementById(
      "text-input",
    ) as HTMLTextAreaElement | null;
    if (textInputEl) textInputEl.value = "";
    textSequences.current = "";

    const fileInputEl = document.getElementById(
      "file-input",
    ) as HTMLInputElement | null;
    if (fileInputEl) fileInputEl.value = "";
    fileSequences.current = "";
    setFilePath("");
  };

  // Analyse the text or file sequences, save them as a run, then show the runs list
  const analyseAndSave = async (input: "text" | "file") => {
    setError("");
    setIsRunning(true);
    try {
      const records =
        input === "text"
          ? await analyseTextSequences(textSequences.current, seqFormat)
          : await analyseFileSequences(fileSequences.current, seqFormat);
      await saveRun(seqFormat, records);
      router.push("/runs");
    } catch (e) {
      setError(`Couldn't analyse and save the sequences: ${e}`);
      setIsRunning(false);
    }
  };

  const submit = () => {
    if (isRunning) return;
    if (textSequences.current && fileSequences.current) {
      alert("You may only send either text or a file. Not both.");
    } else if (textSequences.current) {
      analyseAndSave("text");
    } else if (fileSequences.current) {
      analyseAndSave("file");
    } else {
      alert("Please give either text or a file.");
    }
  };

  return (
    <>
      <Toolbar title="Import" subtitle={filePath || "No file selected"}>
        <Button
          variant="outline"
          size="sm"
          onClick={clearInputs}
          disabled={isRunning}
        >
          Clear
        </Button>
        <Button size="sm" onClick={submit} disabled={isRunning}>
          Submit
        </Button>
      </Toolbar>

      <div className="relative min-h-0 flex-1 overflow-y-auto">
        <DnaMotif
          variant="texture"
          size={54}
          className="pointer-events-none absolute inset-x-0 top-0 w-full"
        />
        <div className="relative mx-auto flex w-full max-w-2xl flex-col gap-6 px-4 py-10">
          {isRunning ? (
            <LoadingIndicator message="Analysing and saving sequences..." />
          ) : null}
          {error ? (
            <p role="alert" className="text-sm text-destructive">
              {error}
            </p>
          ) : null}

          <FormatToggle
            id="format-switch"
            title="Sequence type"
            value={seqFormat}
            checked={seqFormat === "fastq"}
            onCheckedChange={handleFormatSwitch}
          />

          <Accordion>
            <AccordionItem value="text">
              <AccordionTrigger>Input Text</AccordionTrigger>
              <AccordionContent>
                <TextInput
                  id="text-input"
                  title={`Paste ${seqFormat.toUpperCase()}`}
                  onChange={handleTextInput}
                />
              </AccordionContent>
            </AccordionItem>

            <AccordionItem value="file">
              <AccordionTrigger>Input File</AccordionTrigger>
              <AccordionContent>
                <FileInput
                  id="file-input"
                  title={`Upload ${seqFormat.toUpperCase()} file`}
                  onClick={handleFileInput}
                />
              </AccordionContent>
            </AccordionItem>
          </Accordion>
        </div>
      </div>
    </>
  );
}
