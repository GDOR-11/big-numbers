const query_string = window.location.search;
const query = query_string.slice(1).split("&").map(string => string.split("="));
const number = parseInt(query.find(arg => arg[0] == "factorial")?.[1]);
const base = parseInt(query.find(arg => arg[0] == "base")?.[1]) || 10;

if(isNaN(number)) {
    alert("please provide a valid number to view the factorial of");
    throw "hello there, why are you peeking in the console";
}

document.title = `${number}!` + (base == 10 ? `` : ` base ${base}`);

const text_p = document.getElementById("factorial");
async function update_text(text) {
    text_p.innerText = text;
    await new Promise(r => setTimeout(r, 1000));
}

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

// be aware that this takes a few seconds to run
function estimate_bigint_toString_time(bit_length, base) {
    let time = 0;
    let small_bit_length = 1;
    for(;time < 1000;small_bit_length <<= 1) {
        let small_bigint = 1n << BigInt(small_bit_length);
        let start = performance.now();
        small_bigint.toString(base);
        time = performance.now() - start;
    }

    // this estimation can be done because BigInt.prototype.toString runs in O(nlog n) time according to my measurements
    return time * (bit_length * Math.log(bit_length)) / (small_bit_length * Math.log(small_bit_length));
}

async function arraybuffer_to_string(array_buffer, base) {
    const data_view = new DataView(array_buffer);

    await update_text("converting buffer to string...");
    let str = "0x";

    let offset = 0;
    for(;offset < data_view.byteLength - 3;offset += 4) {
        str += u32_to_hex(data_view.getUint32(offset));
    }
    for(;offset < data_view.byteLength;offset++) {
        str += u8_to_hex(data_view.getUint8(offset));
    }

    if(base == 16) {
        return str.slice(2);
    } else {
        await update_text("parsing string into BigInt...");
        let bigint = BigInt(str);

        await update_text("measuring device performance...");
        let estimated_time = estimate_bigint_toString_time((str.length - 2) * 4, base);

        await update_text(`converting bigint into base ${base} string...\nestimated time: ${Math.round(estimated_time) / 1000}s`);
        return bigint.toString(base);
    }
}

async function get_factorial(number, base) {
    await update_text("fetching data...");
    try {
        const response = await fetch(`../binary-bigints/${number}/${number}.bigint`);
        const blob = await response.blob();
        const array_buffer = await blob.arrayBuffer();
        return arraybuffer_to_string(array_buffer, base);
    } catch(err) {
        const response = await fetch(`../decimal-bigints/${number}/${number}.txt`);
        return await response.text();
    } finally {
        await update_text("number not found, check if you haven't made a typo");
    }
}

const factorial = await get_factorial(number, base);

await update_text("displaying...");
text_p.innerText = factorial;
