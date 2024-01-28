const query_string = window.location.search;
const query = query_string.slice(1).split("&").map(string => string.split("="));
const number = parseInt(query.find(arg => arg[0] == "factorial")[1]);
const base = parseInt(query.find(arg => arg[0] == "base")?.[1]) || 10;

if(isNaN(number) || isNaN(base)) {
    alert("please provide a valid number");
    throw "hello there, why are you peeking in the console";
}

document.title = `${number}!`;

async function get_factorial(number) {
    const response = await fetch(`../factorials/${number}/${number}.fctr`);
    const blob = await response.blob();
    const array_buffer = await blob.arrayBuffer();
    const buffer = new Uint8Array(array_buffer);

    console.log("got here");

    let factorial = 0n;
    for(let i = 0;i < buffer.length;i++) {
        if(i % 10000 == 0) console.log(i / number);
        factorial <<= 8n;
        factorial += BigInt(buffer[i]);
    }

    console.log("got here");

    return factorial.toString(base);
}

get_factorial(number).then(factorial => {
    document.getElementById("factorial").innerText = factorial;
});
