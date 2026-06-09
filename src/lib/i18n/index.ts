import { flatten, translator } from "@solid-primitives/i18n";
import { createContext, createMemo, useContext } from "solid-js";
import { en, type Dictionary } from "./dictionaries/en";

type FlatDict = ReturnType<typeof flatten<Dictionary>>;

export const I18nContext =
	createContext<ReturnType<typeof translator<FlatDict>>>();

export function createAppI18n() {
	const dict = createMemo(() => flatten(en));
	const t = translator(dict);
	return { t };
}

export function useT() {
	const ctx = useContext(I18nContext);
	if (!ctx) throw new Error("missing I18nContext");
	return ctx;
}

export type { Dictionary };
