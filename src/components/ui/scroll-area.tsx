import { splitProps, type Component, type JSX } from "solid-js";
import { cn } from "../../lib/utils";

export const ScrollArea: Component<JSX.HTMLAttributes<HTMLDivElement>> = (
	props,
) => {
	const [local, rest] = splitProps(props, ["class"]);
	return <div class={cn("overflow-y-auto", local.class)} {...rest} />;
};
