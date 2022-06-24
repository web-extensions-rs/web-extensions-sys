console.debug('Load WASM server module..');
import wasm_bindgen from './pkg/background_script.js';
wasm_bindgen("./pkg/background_script_bg.wasm")
  .then(module => {
    console.info('WASM loaded');
    module.start();
  })
  .catch(console.error);
