import { instantiate, patchver } from "./lib/patchver_wasm.generated.js";

await instantiate();

export { patchver };
