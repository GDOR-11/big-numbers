import view_factorial from "./pkg/view_factorial.js";

const query_string = window.location.search;
const query = query_string.slice(1).split("&").map(string => string.split("="));
const number = parseInt(query.find(arg => arg[0] == "factorial")[1]);
const base = parseInt(query.find(arg => arg[0] == "base")?.[1]) || 10;

if(isNaN(number)) {
    alert("please provide a valid number to view the factorial of");
    throw "hello there, why are you peeking in the console";
}

document.title = `${number}!` + base == 10 ? "" : ` base ${base}`;

const text_p = document.getElementById("factorial");
async function update_text(text) {
    text_p.innerText = text;
    await new Promise(r => setTimeout(r, 0));
}

async function get_factorial(number, base) {
    await update_text("fetching data...");
    const response = await fetch(`../factorials/${number}/${number}.fctr`);
    const blob = await response.blob();
    const array_buffer = await blob.arrayBuffer();
    const base256 = new Uint8Array(array_buffer);

    await update_text(`converting buffer to base ${base} string...`)
    return view_factorial.base256_to_string(base256, base);
}

const factorial = await get_factorial(number, base);

await update_text("displaying...");
text_p.innerText = factorial;
