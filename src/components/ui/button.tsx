import { cva, type VariantProps } from "class-variance-authority";
import { splitProps, type Component, type JSX } from "solid-js";
import { cn } from "../../lib/utils";

const buttonVariants = cva(
	"inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-zinc-400 disabled:pointer-events-none disabled:opacity-50",
	{
		variants: {
			variant: {
				default: "bg-zinc-100 text-zinc-900 hover:bg-zinc-200",
				destructive: "bg-red-600 text-white hover:bg-red-500",
				ghost: "hover:bg-zinc-800 hover:text-zinc-100",
				outline: "border border-zinc-700 bg-transparent hover:bg-zinc-800",
			},
			size: {
				default: "h-9 px-3 py-2",
				sm: "h-8 rounded-md px-2 text-xs",
				icon: "h-8 w-8",
			},
		},
		defaultVariants: {
			variant: "default",
			size: "default",
		},
	},
);

export interface ButtonProps
	extends
		JSX.ButtonHTMLAttributes<HTMLButtonElement>,
		VariantProps<typeof buttonVariants> {}

export const Button: Component<ButtonProps> = (props) => {
	const [local, rest] = splitProps(props, ["class", "variant", "size"]);
	return (
		<button
			class={cn(
				buttonVariants({ variant: local.variant, size: local.size }),
				local.class,
			)}
			{...rest}
		/>
	);
};
