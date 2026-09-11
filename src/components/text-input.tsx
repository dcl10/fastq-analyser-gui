"use client";

import type { ChangeEvent } from "react";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";

interface TextInputProps {
  id: string;
  title: string;
  onChange: (event: ChangeEvent<HTMLTextAreaElement>) => void;
  isInvalid?: boolean;
}

export function TextInput({ id, title, onChange, isInvalid = false }: TextInputProps) {
  return (
    <div className="flex flex-col gap-1.5">
      <Label htmlFor={id}>{title}</Label>
      <Textarea
        id={id}
        placeholder="Paste one or more sequence records."
        onChange={onChange}
        aria-invalid={isInvalid}
      />
      {isInvalid && (
        <p className="text-sm text-destructive">
          Please paste at least one valid sequence record.
        </p>
      )}
    </div>
  );
}
