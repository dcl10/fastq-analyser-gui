export { cn } from "cn"

const pluralRules = new Intl.PluralRules("en");

// Format a count with the matching word form: "1 record", "2 records", "1,234 records"
export function pluralise(count: number, singular: string, plural = `${singular}s`): string {
  const word = pluralRules.select(count) === "one" ? singular : plural;
  return `${count.toLocaleString()} ${word}`;
}
