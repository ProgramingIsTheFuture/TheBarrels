import { v4 } from "https://deno.land/std@0.116.0/uuid/mod.ts";
import { crypto } from "https://deno.land/std@0.116.0/crypto/mod.ts";

/*
 * Generate a new uuid
 * Returns empty string if anything fails
 */
export const generateUUID = (): string => {
	const uuid: string | undefined = crypto.randomUUID?.();

	if (!uuid) {
		return "";
	}

	return v4.validate(uuid) ? uuid : "";
};

const encodePassword = async (password: string): Promise<ArrayBuffer> => {
	const cr: ArrayBuffer = await crypto.subtle
		.digest("SHA-256", new TextEncoder().encode(password))
		.then((r: ArrayBuffer) => r);

	return Promise.resolve(cr);
};

/*
 * Receives password and returns the hash version
 */

export const hashPassword = async (password: string): Promise<string> => {
	const encoded: ArrayBuffer = await encodePassword(password).then((r) => r);

	const sha = new Uint8Array(encoded).reduce(
		(str, byte) => str + byte.toString(16).padStart(2, "0"),
		""
	);

	return sha;
};

/*
 * Receives a username and a hashed password
 */
const createUser = (username: string, password: string) => {};
