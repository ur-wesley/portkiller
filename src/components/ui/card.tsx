import { splitProps, type Component, type JSX } from "solid-js";
import { cn } from "../../lib/utils";

export const Card: Component<JSX.HTMLAttributes<HTMLDivElement>> = (props) => {
	const [local, rest] = splitProps(props, ["class"]);
	return (
		<div
			class={cn(
				"glass-surface rounded-lg text-zinc-100",
				local.class,
			)}
			{...rest}
		/>
	);
};

export const CardContent: Component<JSX.HTMLAttributes<HTMLDivElement>> = (
	props,
) => {
	const [local, rest] = splitProps(props, ["class"]);
	return <div class={cn("p-4", local.class)} {...rest} />;
};
