console.log("dio cane");

(async () => {
    // Note: files in `crate/pkg/` will be created on the first build.
    await import("../dist/index");
})();