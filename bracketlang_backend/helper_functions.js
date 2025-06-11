export function js_print(value) {
    let output = document.getElementById("bl-out");

    output.innerHTML = `<p class="console-response">${value}</p>` + output.innerHTML;
}

export function js_get_time() {
    const date = new Date();
    return date.getTime();
}