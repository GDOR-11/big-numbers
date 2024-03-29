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

async function binary_number_to_string(number, base) {
    let response = await fetch(`../binary-bigints/${number}/${number}.bigint`);
    if(!response.ok) return "";
    let blob = await response.blob(); response = null;
    let array_buffer = await blob.arrayBuffer(); blob = null;
    let data_view = new DataView(array_buffer); array_buffer = null;

    await update_text("converting buffer to hex string...");
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
        await update_text("converting hex string into bigint...");
        let bigint = BigInt(string);

        await update_text(`converting bigint into base ${base} string...`);
        return bigint.toString(base);
    }
}

async function get_decimal_string(number) {
    let response = await fetch(`../decimal-bigints/${number}/${number}.txt`);
    if(!response.ok) return "";
    return response.text();
}

async function get_number(number, base) {
    await update_text("fetching data...");
    let string = "";
    if(base == 10) string = await get_decimal_string(number);
    if(string == "") string = await binary_number_to_string(number, base);
    if(string == "") await update_text("number not found, check if you haven't made a typo");
    return string;
}

const big_number = await get_number(number, base);

if(big_number != "") {
    await update_text("displaying...");
    text_p.innerText = big_number;
}
