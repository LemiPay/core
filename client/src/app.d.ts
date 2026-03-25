// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

// Returns __T__ if __OK__ and __B__ if __Error__
type Result<T, B> = Promise<T | B>;

export type { Result };
