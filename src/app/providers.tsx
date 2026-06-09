import { QueryClientProvider } from "@tanstack/solid-query";
import type { ParentComponent } from "solid-js";
import { createAppI18n, I18nContext } from "../lib/i18n";
import { createAppQueryClient } from "../lib/query/client";

const queryClient = createAppQueryClient();
const { t } = createAppI18n();

export const Providers: ParentComponent = (props) => (
	<QueryClientProvider client={queryClient}>
		<I18nContext.Provider value={t}>{props.children}</I18nContext.Provider>
	</QueryClientProvider>
);
