const query_string = window.location.search;
const query = query_string.slice(1).split("&").map(string => string.split("="));
const number = parseInt(query.find(arg => arg[0] == "factorial")[1]);

if(isNaN(number)) {
    alert("please provide a valid number to view");
}

document.title = `${number}!`;

async function get_factorial(number) {
    const response = await fetch(`../factorials/${number}/${number}.fctr`);
    const blob = await response.blob();
    const array_buffer = await blob.arrayBuffer();
    const buffer = new Uint8Array(array_buffer);

    let factorial = 0n;
    let digit_significance = 1n;
    for(let i = buffer.length - 1;i >= 0;i--) {
        factorial += BigInt(buffer[i]) * digit_significance;
        digit_significance *= 256n;
    }

    return factorial.toString();
}

get_factorial(number).then(factorial => {
    document.getElementById("factorial").innerText = factorial;
});
