import init, { evaluate_string, create_default_env } from "./pkg/bracketlang_backend.js";

// HELLO THERE

init().then(() => {
    let env = create_default_env();
    let inputField = document.getElementById("bl-input");
    let output = document.getElementById("bl-out");
    let form = document.getElementById("bl-form");

    form.addEventListener("submit", (event) => {
        event.preventDefault();
        output.innerHTML = `<p class="console-prompt">=> ${inputField.value}</p>` + output.innerHTML;
        evaluate_string(inputField.value, env);
        inputField.value = "";
    })
})