import { patchver } from "./mod.ts";

Deno.test("patchver", () => {
   const selfExe = Deno.execPath();
   const input = Deno.readFileSync(selfExe);
   const channel = "rc";

   const output = patchver(input, channel);
   console.log(output.byteLength);
});
