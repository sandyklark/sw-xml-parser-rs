// require must be used here as esm doesn't support compiled binaries
const {fromUrl} = require('../index');

(async () => {
  let url = 'https://www.githubstatus.com/history.atom';
  let res = await fromUrl(url);
  console.log(res);
})();
