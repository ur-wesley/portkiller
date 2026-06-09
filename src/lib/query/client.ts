import { QueryClient } from "@tanstack/solid-query";

export function createAppQueryClient() {
	return new QueryClient({
		defaultOptions: {
			queries: {
				staleTime: 2_000,
				retry: 1,
			},
		},
	});
}
