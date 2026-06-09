import { splitProps, type Component, type JSX } from "solid-js";
import { cn } from "../../lib/utils";

export interface SwitchProps extends Omit<
	JSX.HTMLAttributes<HTMLButtonElement>,
	"onChange"
> {
	checked?: boolean;
	onChange?: (checked: boolean) => void;
}

export const Switch: Component<SwitchProps> = (props) => {
	const [local, rest] = splitProps(props, ["class", "checked", "onChange"]);
	return (
		<button
			type="button"
			role="switch"
			aria-checked={local.checked}
			class={cn(
				"relative inline-flex h-5 w-9 shrink-0 cursor-pointer rounded-full border border-zinc-600 transition-colors",
				local.checked ? "bg-emerald-600" : "bg-zinc-800",
				local.class,
			)}
			onClick={() => local.onChange?.(!local.checked)}
			{...rest}
		>
			<span
				class={cn(
					"pointer-events-none block h-4 w-4 rounded-full bg-white shadow transition-transform",
					local.checked ? "translate-x-4" : "translate-x-0.5",
				)}
			/>
		</button>
	);
};
