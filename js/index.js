require('../css/base.css');
require('../css/index.css');

(async () => {
    // Note: files in `crate/pkg/` will be created on the first build.
    await import("../dist/index");
})();
