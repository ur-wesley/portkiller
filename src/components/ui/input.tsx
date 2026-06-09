import { splitProps, type Component, type JSX } from "solid-js";
import { cn } from "../../lib/utils";

export interface InputProps extends JSX.InputHTMLAttributes<HTMLInputElement> {}

export const Input: Component<
	InputProps & { ref?: (el: HTMLInputElement) => void }
> = (props) => {
	const [local, rest] = splitProps(props, ["class", "type", "ref"]);
	return (
		<input
			ref={local.ref}
			type={local.type ?? "text"}
			class={cn(
				"flex h-9 w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1 text-sm text-zinc-100 placeholder:text-zinc-500 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-zinc-500",
				local.class,
			)}
			{...rest}
		/>
	);
};
