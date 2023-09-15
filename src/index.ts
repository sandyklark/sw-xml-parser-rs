// require must be used here as esm doesn't support compiled binaries
const {fromUrl} = require('../index');

(async () => {
  let res = await fromUrl();
  console.log(res);
})();
