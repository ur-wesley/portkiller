import {
	createContext,
	createSignal,
	splitProps,
	useContext,
	type Component,
	type JSX,
} from "solid-js";
import { cn } from "../../lib/utils";

const TabsContext = createContext<{
	value: () => string;
	setValue: (v: string) => void;
}>();

export function useTabs() {
	const ctx = useContext(TabsContext);
	if (!ctx) throw new Error("useTabs must be used within Tabs");
	return ctx;
}

export const Tabs: Component<{
	defaultValue: string;
	class?: string;
	children: JSX.Element;
}> = (props) => {
	const [value, setValue] = createSignal(props.defaultValue);
	return (
		<TabsContext.Provider value={{ value, setValue }}>
			<div class={cn("flex h-full min-h-0 flex-col", props.class)}>
				{props.children}
			</div>
		</TabsContext.Provider>
	);
};

export const TabsList: Component<JSX.HTMLAttributes<HTMLDivElement>> = (
	props,
) => {
	const [local, rest] = splitProps(props, ["class"]);
	return (
		<div
			class={cn("flex gap-1 border-b border-zinc-800 px-3 pt-3", local.class)}
			{...rest}
		/>
	);
};

export const TabsTrigger: Component<{
	value: string;
	class?: string;
	children: JSX.Element;
}> = (props) => {
	const ctx = useContext(TabsContext)!;
	const active = () => ctx.value() === props.value;
	return (
		<button
			type="button"
			class={cn(
				"rounded-t-md px-3 py-2 text-sm transition-colors",
				active()
					? "bg-zinc-900 text-zinc-100"
					: "text-zinc-400 hover:text-zinc-200",
				props.class,
			)}
			onClick={() => ctx.setValue(props.value)}
		>
			{props.children}
		</button>
	);
};

export const TabsContent: Component<{
	value: string;
	class?: string;
	children: JSX.Element;
}> = (props) => {
	const ctx = useContext(TabsContext)!;
	return (
		<div
			class={cn(
				"min-h-0 flex-1 overflow-hidden",
				props.class,
				ctx.value() !== props.value && "hidden",
			)}
		>
			{props.children}
		</div>
	);
};
