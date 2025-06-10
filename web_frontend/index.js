import init, { evaluate_string, create_default_env } from "./pkg/bracketlang_backend.js";

init().then(() => {
    let env = create_default_env();
    let inputButton = document.getElementById("bl-button");
    let inputField = document.getElementById("bl-input");

    inputButton.addEventListener("click", (event) => {
        evaluate_string(inputField.value, env);
        inputField.value = "";
    })
})