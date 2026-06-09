import { render } from "solid-js/web";
import App from "./app/App";
import { Providers } from "./app/providers";
import "./index.css";

const root = document.getElementById("root");
if (!root) throw new Error("root element not found");

render(
	() => (
		<Providers>
			<App />
		</Providers>
	),
	root,
);
