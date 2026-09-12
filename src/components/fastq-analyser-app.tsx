"use client";

import { useRef, useState, type ChangeEvent } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { Button } from "@/components/ui/button";
import { FastaResultPanel } from "@/components/fasta-result-panel";
import { FastqResultPanel } from "@/components/fastq-result-panel";
import { FileInput } from "@/components/file-input";
import { FormatToggle } from "@/components/format-toggle";
import { LoadingIndicator } from "@/components/loading-indicator";
import { ResultsDialog } from "@/components/results-dialog";
import { TextInput } from "@/components/text-input";
import { ThemeSwitch } from "@/components/theme-switch";
import { Wordmark } from "@/components/brand/wordmark";
import { analyseFileSequences, analyseTextSequences } from "@/lib/analysis";
import type { SeqFormat, SeqResult } from "@/types/results";

export function FastqAnalyserApp() {
  const textSequences = useRef("");
  const fileSequences = useRef("");
  const [seqFormat, setSeqFormat] = useState<SeqFormat>("fastq");
  const [results, setResults] = useState<SeqResult[]>([]);
  const [isOpen, setIsOpen] = useState(false);

  // Change the text sequences in state
  const handleTextInput = (event: ChangeEvent<HTMLTextAreaElement>) => {
    textSequences.current = event.target.value;
  };

  // Change the file sequences in state
  const handleFileInput = async () => {
    const filePath = await open({
      directory: false,
      multiple: false,
      filters: [
        {
          name: "Sequence files",
          extensions: ["fq", "fastq", "fa", "fasta"],
        },
      ],
    });

    const fileInputEl = document.getElementById("file-input") as HTMLInputElement | null;
    if (fileInputEl) fileInputEl.value = filePath ?? "";
    fileSequences.current = filePath ?? "";
  };

  // Change the sequence format
  const handleFormatSwitch = (checked: boolean) => {
    setSeqFormat(checked ? "fastq" : "fasta");
  };

  // Clear the input fields and reset the state
  const clearInputs = () => {
    const textInputEl = document.getElementById("text-input") as HTMLTextAreaElement | null;
    if (textInputEl) textInputEl.value = "";
    textSequences.current = "";

    const fileInputEl = document.getElementById("file-input") as HTMLInputElement | null;
    if (fileInputEl) fileInputEl.value = "";
    fileSequences.current = "";
  };

  // Send the text sequences to the backend and return the analytics
  const analyseText = async () => {
    setIsOpen(true);
    const analysed = await analyseTextSequences(textSequences.current, seqFormat);
    setResults(analysed);
  };

  // Send the file sequences to the backend and return the analytics
  const analyseFile = async () => {
    setIsOpen(true);
    const analysed = await analyseFileSequences(fileSequences.current, seqFormat);
    setResults(analysed);
  };

  // Clear the results when the dialog is closed
  const closeAndClearResults = () => {
    clearInputs();
    setResults([]);
    setIsOpen(false);
  };

  return (
    <div className="mx-auto flex w-full max-w-2xl flex-col gap-6 px-4 py-10">
      <ResultsDialog title="Results" isOpen={isOpen} onClose={closeAndClearResults}>
        {results.length > 0 ? (
          <Accordion>
            {results.map((result, index) => (
              <AccordionItem key={`${result.id}-${index}`} value={String(index)}>
                <AccordionTrigger>{result.id}</AccordionTrigger>
                <AccordionContent>
                  {result.result_type === "fastq" ? (
                    <FastqResultPanel result={result} />
                  ) : (
                    <FastaResultPanel result={result} />
                  )}
                </AccordionContent>
              </AccordionItem>
            ))}
          </Accordion>
        ) : (
          <LoadingIndicator message="Loading results..." />
        )}
      </ResultsDialog>

      <div className="flex items-center justify-between">
        <Wordmark size={22} />
        <ThemeSwitch />
      </div>

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
            <TextInput id="text-input" title="Paste fastq" onChange={handleTextInput} />
          </AccordionContent>
        </AccordionItem>

        <AccordionItem value="file">
          <AccordionTrigger>Input File</AccordionTrigger>
          <AccordionContent>
            <FileInput id="file-input" title="Upload Fastq file" onClick={handleFileInput} />
          </AccordionContent>
        </AccordionItem>
      </Accordion>

      <div className="flex justify-center gap-4">
        <Button
          onClick={() => {
            if (textSequences.current && fileSequences.current) {
              alert("You may only send either text or a file. Not both.");
            } else if (textSequences.current) {
              analyseText();
            } else if (fileSequences.current) {
              analyseFile();
            } else {
              alert("Please give either text or a file.");
            }
          }}
        >
          Submit
        </Button>
        <Button variant="outline" onClick={clearInputs}>
          Clear
        </Button>
      </div>
    </div>
  );
}
