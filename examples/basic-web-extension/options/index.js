console.debug('Load WASM options module..');
import wasm_bindgen from './pkg/options.js';
wasm_bindgen("./pkg/options_bg.wasm")
  .then(module => {
    console.info('WASM loaded');
    module.start();
  })
  .catch(console.error);
