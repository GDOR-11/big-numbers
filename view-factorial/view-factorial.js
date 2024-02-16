const query = window.location.search.slice(1).split("&").map(string => string.split("="));
const number = query.find(arg => arg[0] == "number")?.[1];
const base = parseInt(query.find(arg => arg[0] == "base")?.[1]) || 10;

if(!number) {
    alert("please provide a valid number to view");
    throw "hello there, why are you peeking in the console";
}

document.title = `${number}` + (base == 10 ? `` : ` base ${base}`);

const text_p = document.getElementById("big-number");
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

async function binary_number_to_string(number, base) {
    const response = await fetch(`../binary-bigints/${number}/${number}.bigint`);
    const blob = await response.blob(); response = null;
    const array_buffer = await blob.arrayBuffer(); blob = null;
    const data_view = new DataView(array_buffer); array_buffer = null;

    await update_text("converting buffer to string...");
    let string = "0x";

    let offset = 0;
    for(;offset < data_view.byteLength - 3;offset += 4) {
        string += u32_to_hex(data_view.getUint32(offset));
    }
    for(;offset < data_view.byteLength;offset++) {
        string += u8_to_hex(data_view.getUint8(offset));
    }

    data_view = null;

    if(base == 16) {
        return string.slice(2);
    } else {
        await update_text("parsing string into BigInt...");
        let bigint = BigInt(string);
        const bit_length = (string.length - 2) * 4; string = null;

        await update_text("measuring device performance...");
        let estimated_time = estimate_bigint_toString_time(bit_length, base);

        await update_text(`converting bigint into base ${base} string...\nestimated time: ${Math.round(estimated_time) / 1000}s`);
        return bigint.toString(base);
    }
}

async function decimal_number_to_string(number, base) {
    const response = await fetch(`../decimal-bigints/${number}/${number}.txt`);
    const text = response.text(); response = null;

    // it makes no sense to do nothing
    // this single line makes 90% of use cases a LOT more efficient
    if(base == 10) return text;

    await update_text(`converting from base ${base} string to bigint...`);
    let bigint = BigInt(text); text = null;

    await update_text(`converting from bigint to base ${base} string`);
    return bigint.toString(base);
}

async function get_number(number, base) {
    await update_text("fetching data...");
    try {
        return await binary_number_to_string(number, base);
    } catch(err) {
        return await decimal_number_to_string(number, base);
    } finally {
        await update_text("number not found, check if you haven't made a typo");
    }
}

const big_number = await get_number(number, base);

await update_text("displaying...");
text_p.innerText = big_number;
