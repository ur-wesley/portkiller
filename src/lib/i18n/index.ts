import { flatten, resolveTemplate, translator } from "@solid-primitives/i18n";
import { createContext, useContext } from "solid-js";
import { en, type Dictionary } from "./dictionaries/en";

type FlatDict = ReturnType<typeof flatten<Dictionary>>;

export const I18nContext =
	createContext<ReturnType<typeof translator<FlatDict>>>();

const flatDict = flatten(en);

export function createAppI18n() {
	const t = translator(() => flatDict, resolveTemplate);
	return { t };
}

export function useT() {
	const ctx = useContext(I18nContext);
	if (!ctx) throw new Error("missing I18nContext");
	return ctx;
}

export type { Dictionary };
