import init, { evaluate_string, create_default_env } from "./pkg/bracketlang_backend.js";

init().then(() => {
    let env = create_default_env();
    let inputButton = document.getElementById("bl-button");
    let inputField = document.getElementById("bl-input");

    console.log(inputButton)

    inputButton.addEventListener("click", (event) => {
        console.log(evaluate_string(inputField.value, env));
    })
})