const query_string = window.location.search;
const query = query_string.slice(1).split("&").map(string => string.split("="));
const number = parseInt(query.find(arg => arg[0] == "factorial")[1]);
const base = parseInt(query.find(arg => arg[0] == "base")?.[1]) || 10;

if(isNaN(number)) {
    alert("please provide a valid number to view the factorial of");
    throw "hello there, why are you peeking in the console";
}

document.title = `${number}!`;

// lol
function u32_to_hex(num) {
    return String.fromCharCode(
        ((num >>> 28)     ) + (  num       < 2684354560 ? 48 : 87),
        ((num >>> 24) & 15) + (((num >>> 24) & 15) < 10 ? 48 : 87),
        ((num >>> 20) & 15) + (((num >>> 20) & 15) < 10 ? 48 : 87),
        ((num >>> 16) & 15) + (((num >>> 16) & 15) < 10 ? 48 : 87),
        ((num >>> 12) & 15) + (((num >>> 12) & 15) < 10 ? 48 : 87),
        ((num >>>  8) & 15) + (((num >>>  8) & 15) < 10 ? 48 : 87),
        ((num >>>  4) & 15) + (((num >>>  4) & 15) < 10 ? 48 : 87),
        ((num       ) & 15) + (((num       ) & 15) < 10 ? 48 : 87)
    );
}
function u8_to_hex(num) {
    return String.fromCharCode(
        (num >>> 4) + ( num      < 160 ? 48 : 87),
        (num & 15)  + ((num & 15) < 10 ? 48 : 87)
    );
}
async function get_factorial(number, base) {
    const response = await fetch(`../factorials/${number}/${number}.fctr`);
    const blob = await response.blob();
    const array_buffer = await blob.arrayBuffer();
    const data_view = new DataView(array_buffer);

    let string = "0x";
    let i = 0;
    for(;i <= data_view.byteLength - 4;i += 4) {
        string += u32_to_hex(data_view.getUint32(i));
    }
    for(;i < data_view.byteLength;i++) {
        string += u8_to_hex(data_view.getUint8(i));
    }
    const factorial = BigInt(string);

    return factorial.toString(base);
}

document.getElementById("factorial").innerText = "loading... (this might take a few seconds)";
get_factorial(number, base).then(factorial => {
    document.getElementById("factorial").innerText = factorial;
});
