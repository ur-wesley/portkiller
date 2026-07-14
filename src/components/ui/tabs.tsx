import { Tabs as KobalteTabs, useTabsContext } from "@kobalte/core/tabs";
import {
	splitProps,
	type Component,
	type JSX,
} from "solid-js";
import { cn } from "../../lib/utils";

export function useTabs() {
	const ctx = useTabsContext();
	return {
		value: () => String(ctx.listState().selectedKey() ?? ""),
		setValue: (v: string) => {
			ctx.listState().selectionManager().setSelectedKeys([v]);
		},
	};
}

export const Tabs: Component<{
	defaultValue?: string;
	value?: string;
	onChange?: (value: string) => void;
	class?: string;
	children: JSX.Element;
}> = (props) => {
	const [local, rest] = splitProps(props, ["class", "children"]);
	return (
		<KobalteTabs
			class={cn("flex h-full min-h-0 flex-col", local.class)}
			{...rest}
		>
			{local.children}
		</KobalteTabs>
	);
};

export const TabsList: Component<JSX.HTMLAttributes<HTMLDivElement>> = (
	props,
) => {
	const [local, rest] = splitProps(props, ["class"]);
	return (
		<KobalteTabs.List
			class={cn(
				"flex h-10 w-full items-center rounded-lg border border-white/8 bg-zinc-900/45 p-1 text-zinc-400 backdrop-blur-md",
				local.class,
			)}
			{...rest}
		/>
	);
};

export const TabsTrigger: Component<{
	value: string;
	class?: string;
	children: JSX.Element;
}> = (props) => {
	const [local, rest] = splitProps(props, ["class", "children", "value"]);
	return (
		<KobalteTabs.Trigger
			value={local.value}
			class={cn(
				"flex h-8 w-full min-w-0 items-center justify-center whitespace-nowrap rounded-md px-3 text-sm font-medium transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-zinc-500 disabled:pointer-events-none disabled:opacity-50 data-[selected]:bg-zinc-950/75 data-[selected]:text-zinc-100 data-[selected]:shadow-sm",
				local.class,
			)}
			{...rest}
		>
			{local.children}
		</KobalteTabs.Trigger>
	);
};

export const TabsContent: Component<{
	value: string;
	class?: string;
	children: JSX.Element;
}> = (props) => {
	const [local, rest] = splitProps(props, ["class", "children", "value"]);
	return (
		<KobalteTabs.Content
			value={local.value}
			class={cn(
				"min-h-0 flex-1 overflow-hidden focus-visible:outline-none",
				local.class,
			)}
			{...rest}
		>
			{local.children}
		</KobalteTabs.Content>
	);
};
