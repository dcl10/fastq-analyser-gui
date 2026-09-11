"use client";

import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";

interface FormatToggleProps {
  id: string;
  title: string;
  value: string;
  checked: boolean;
  onCheckedChange: (checked: boolean) => void;
}

export function FormatToggle({
  id,
  title,
  value,
  checked,
  onCheckedChange,
}: FormatToggleProps) {
  return (
    <div className="flex items-center gap-2">
      <Label htmlFor={id}>
        {title}: {value}
      </Label>
      <Switch id={id} checked={checked} onCheckedChange={onCheckedChange} />
    </div>
  );
}
