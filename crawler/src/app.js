global.sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
global.dump = object => console.dir(object, { depth: null });
global.fetch = require("node-fetch");

const lists = [
    "https://raw.githubusercontent.com/Anuken/Mindustry/master/servers_be.json",
    "https://raw.githubusercontent.com/Anuken/Mindustry/master/servers_v6.json",
    "https://raw.githubusercontent.com/Anuken/Mindustry/master/servers_v7.json"
];

async function update_server(address) {
    //console.log(`Updating server ${address}`);
    const res = await fetch(`http://app/api/server/${address}`);
    const json = await res.json()
    //console.log(json.error || 'ok');
}

async function update_list(list) {
    //console.log(`Updating list ${list}`);
    const response = await fetch(list);
    const json = await response.json();
    const addresses = json
        .map(e => e.address)
        .flat();
    
    for (let address of addresses) {
        try {
            await update_server(address);
        }
        catch (e) {
            console.error(e);
        }
        await sleep(1000);
    }
}

(async () => {

    let index = 0;
    while (true) {
        try {
            await update_list(lists[index]);
        }
        catch (e) {
            console.error(e);
        }
        await sleep(1000);

        index += 1;
        index %= lists.length;
    }

})();
