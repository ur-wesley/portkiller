import { match } from "ts-pattern";

export function mapKillError(
	error: string,
	messages: { denied: string; failed: string },
) {
	return match(error)
		.with("access_denied", () => messages.denied)
		.otherwise(() => messages.failed);
}
