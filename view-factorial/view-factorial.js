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


    // the main bottleneck is getting the buffer data and turning it into a BigInt
    let factorial = 0n;
    let significance = 0n;
    for(let i = buffer.length - 1;i >= 0;i--) {
        if(i % 10000 == 0) console.log(i / buffer.length);
        factorial |= BigInt(buffer[i]) << significance;
        significance += 8n;
    }

    return factorial.toString(base);
}

get_factorial(number).then(factorial => {
    document.getElementById("factorial").innerText = factorial;
});
