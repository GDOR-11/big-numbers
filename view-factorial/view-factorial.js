import init_wasm, { base256_to_string } from "./pkg/view_factorial.js";

const query_string = window.location.search;
const query = query_string.slice(1).split("&").map(string => string.split("="));
const number = parseInt(query.find(arg => arg[0] == "factorial")?.[1]);

if(isNaN(number)) {
    alert("please provide a valid number to view the factorial of");
    throw "hello there, why are you peeking in the console";
}

document.title = `${number}!`;

const text_p = document.getElementById("factorial");
async function update_text(text) {
    text_p.innerText = text;
    await new Promise(r => setTimeout(r, 1000));
}

async function get_factorial(number) {
    await update_text("fetching data...");
    const response = await fetch(`../factorials/${number}/${number}.fctr`);
    const blob = await response.blob();
    const array_buffer = await blob.arrayBuffer();

    await update_text("processing data...");
    const base256_array = new Uint8Array(array_buffer);

    return base256_to_string(base256_array);
}

await update_text("initializing webassembly...");
await init_wasm();

const factorial = await get_factorial(number);

await update_text("displaying...");
text_p.innerText = factorial;
