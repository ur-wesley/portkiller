import { type Component } from "solid-js";
import { Input } from "../../components/ui/input";

export const SearchBar: Component<{
	value: string;
	placeholder: string;
	onInput: (value: string) => void;
	ref?: (el: HTMLInputElement) => void;
}> = (props) => (
	<Input
		ref={props.ref}
		value={props.value}
		placeholder={props.placeholder}
		onInput={(e) => props.onInput(e.currentTarget.value)}
	/>
);
