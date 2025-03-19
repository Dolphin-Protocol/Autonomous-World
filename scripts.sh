set PROJECT_NAME "autonomous-game"

# Shim to tie the thing together
sed -i '' 's/import \* as __wbg_star0 from '\''env'\'';//' dist/$PROJECT_NAME.js
sed -i '' 's/let wasm;/let wasm; export const set_wasm = (w) => wasm = w;/' dist/$PROJECT_NAME.js
sed -i '' 's/imports\['\''env'\''\] = __wbg_star0;/return imports.wbg;/' dist/$PROJECT_NAME.js
sed -i '' 's/const imports = __wbg_get_imports();/return __wbg_get_imports();/' dist/$PROJECT_NAME.js
