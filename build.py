import os, shutil

os.system("cargo build --all")
shutil.copy("./bracketlang_backend/pkg/bracketlang_backend.js", "./web_frontend/pkg/bracketlang_backend.js")
shutil.copy("./bracketlang_backend/pkg/bracketlang_backend_bg.wasm", "./web_frontend/pkg/bracketlang_backend_bg.wasm")
os.system("py -m http.server -d web_frontend")