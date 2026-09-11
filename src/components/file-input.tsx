"use client";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

interface FileInputProps {
  id: string;
  title: string;
  onClick: () => void;
  isInvalid?: boolean;
}

export function FileInput({ id, title, onClick, isInvalid = false }: FileInputProps) {
  return (
    <div className="flex flex-col gap-1.5">
      <Label htmlFor={id}>{title}</Label>
      <div className="flex gap-2">
        <Button type="button" size="sm" variant="outline" onClick={onClick}>
          Select
        </Button>
        <Input id={id} readOnly aria-invalid={isInvalid} />
      </div>
      {isInvalid ? (
        <p className="text-sm text-destructive">Please select a valid file.</p>
      ) : (
        <p className="text-sm text-muted-foreground">
          Upload a sequence file with DNA sequences.
        </p>
      )}
    </div>
  );
}
