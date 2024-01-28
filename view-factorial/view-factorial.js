const query_string = window.location.search;
const query = query_string.slice(1).split("&").map(string => string.split("="));
const number = parseInt(query.find(arg => arg[0] == "factorial")[1]);
const base = parseInt(query.find(arg => arg[0] == "base")?.[1]) || 10;

if(isNaN(number) || isNaN(base)) {
    alert("please provide a valid number");
    throw "hello there, why are you peeking in the console";
}

document.title = `${number}!`;

function tohex(num) {
    return String.fromCharCode(num >> 4 + (num < 160 ? 48 : 87))
        + String.fromCharCode(num & 15 + (num & 15 < 10 ? 48 : 87));
}
async function get_factorial(number) {
    const response = await fetch(`../factorials/${number}/${number}.fctr`);
    const blob = await response.blob();
    const array_buffer = await blob.arrayBuffer();
    const buffer = new Uint8Array(array_buffer);

    let start = performance.now();

    let string = "0x";
    for(let i = 0;i < buffer.length;i++) {
        string += tohex(buffer[i]);
    }
    const factorial = BigInt(string);

    console.log(performance.now() - start);

    return factorial.toString(base);
}

get_factorial(number).then(factorial => {
    document.getElementById("factorial").innerText = factorial;
});
