import { assertNotEquals } from "https://deno.land/std@0.116.0/testing/asserts.ts";
import { generateUUID, hashPassword } from "./methods.ts";

Deno.test("Generate uuid", () => {
	const uuid = generateUUID();

	console.log("\nThis is the UUID: ", uuid);
	assertNotEquals(uuid, "");
});

Deno.test("Hash Password", async () => {
	const password = "abcd1234";

	const hashPass = await hashPassword(password);

	console.log("\nThe hash is - ", hashPass);
	assertNotEquals(password, hashPassword);
});
