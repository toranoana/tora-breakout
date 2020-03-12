(async function() {
  const wasm = await import("tora-breakout-wasm");
  wasm.start(ASSET_URL);
})();
