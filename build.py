import os, shutil

os.system("wasm-pack build --target web bracketlang_backend")
shutil.copy("./bracketlang_backend/pkg/bracketlang_backend.js", "./web_frontend/pkg/bracketlang_backend.js")
shutil.copytree("./bracketlang_backend/pkg/snippets", "./web_frontend/pkg/snippets", dirs_exist_ok=True)
shutil.copy("./bracketlang_backend/pkg/bracketlang_backend_bg.wasm", "./web_frontend/pkg/bracketlang_backend_bg.wasm")
os.system("py -m http.server -d web_frontend")