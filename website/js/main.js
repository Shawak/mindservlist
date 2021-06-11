(function(){

    function dateDiffStr(date) {
        if (date == 0)
            return 'never';

        let diff = (new Date().getTime() / 1000 - date / 1000);
        if (diff <= 0)
            return 'just now';

        const dateValues = [
            [ " year",   diff / 60 / 60 / 24 / 7 / 4 / 12 ],
            [ " month",  diff / 60 / 60 / 24 / 7 / 4 % 12 ],
            [ " week",   diff / 60 / 60 / 24 / 7 % 4 ],
            [ " day",    diff / 60 / 60 / 24 % 7 ],
            [ "h",       diff / 60 / 60 % 24 ],
            [ "m",       diff / 60 % 60 ],
            [ "s",       diff % 60 ]
        ];

        /*var t = [];
        for (var i = 0; i < dateValues.length; i++) {
            var name = dateValues[i][0];
            var value = Math.floor(dateValues[i][1]);
            if (value > 0)
                t.push(value + ' ' + name + (value != 1 ? 's' : ''));
        }
        var last = t.pop();
        var ret = t.join(', ');
        if (t.length > 1)
            ret += ' and ' + last;
        return ret + ' ago';*/
    
        for (let i = 0; i < dateValues.length; i++) {
            let name = dateValues[i][0];
            let value = Math.floor(dateValues[i][1]);
            if (value > 0)
                return value + name + (name.length > 1 && value > 1 ? 's' : '') + ' ago';
        }

        return '0' + dateValues[dateValues.length - 1][0] + ' ago';
    }

    function escapeHtml(unsafe) {
        return unsafe.toString()
            .replace(/&/g, "&amp;")
            .replace(/</g, "&lt;")
            .replace(/>/g, "&gt;")
            .replace(/"/g, "&quot;")
            .replace(/'/g, "&#039;");
    }

    // https://github.com/Anuken/Arc/blob/ca797d336b86bfe091162b5e5dc73521e04e4817/arc-core/src/arc/graphics/Color.java
    // https://mindustrygame.github.io/wiki/modding/#built-in-colors
    function colors(color) {
        const colors = {
            '': 'white',

            white: 'white',
            lightgray: '#bfbfbfff',
            gray: '#7f7f7fff',
            darkgray: '#3f3f3fff',
            black: 'black',
            clear: 'black',
            
            blue: '#0000ffff',
            navy: '#00007fff',
            royal: '#4169e1ff',
            slate: '#708090ff',
            sky: '#87ceebff',
            cyan: '#00ffffff',
            teal: '#007f7fff',
            
            green: '#00ff00ff',
            acid: '#7fff00ff',
            lime: '#32cd32ff',
            forest: '#228b22ff',
            olive: '#6b8e23ff',
            
            yellow: '#ffff00ff',
            gold: '#ffd700ff',
            goldenrod: '#daa520ff',
            orange: '#ffa500ff',
            
            brown: '#8b4513ff',
            tan: '#d2b48cff',
            brick: '#b22222ff',
            
            red: '#ff0000ff',
            scarlet: '#ff341cff',
            coral: '#ff7f50ff',
            salmon: '#fa8072ff',
            pink: '#ff69b4ff',
            magenta: '#7f007fff',
            
            purple: '#a020f0ff',
            violet: '#ee82eeff',
            maroon: '#b03060ff',

            // alias?
            crimson: '#ff341cff', // scarlet

            // special
            accent: '#ffcb39ff'
        };

        if (color[0] == '#') {
            return color;
        } else if (colors[color.toLowerCase()]) {
            return colors[color];
        }

        return undefined;
    }

    function renderColor(str) {
        return str
            .replace(/\[([a-zA-Z0-9#]*?)\](.*?)(?=(\[|\n|\]|$))/g, (_match, color, text) => {
                let resolved_color = colors(color);
                if (resolved_color === undefined) {
                    if (window.location.hash == '#dev') {
                        console.error(`unknown color: ${color} in text "${_match}"`);
                    }
                    return _match;
                }
                return `<font style="color: ${resolved_color}">${text}</font>`;
            });
    }

    function renderString (data, type, row) {
        return renderColor(escapeHtml(data))
    }

    function renderAddress (data, type, row) {
        let ip = (data.endsWith(':6567') ? data.substr(0, data.length - 5) : data);
        return `<a onclick="navigator.clipboard.writeText(this.text)" href="mindustry://${ip}">${ip}</a>`;
    }

    /*function renderUpdated (data, type, row) {
        return (type == 'sort' || type == 'type') ? parseInt(data) : dateDiffStr(data * 1000);
    }*/

    function renderLastSeen (data, type, row) {
        if (type == 'sort' || type == 'type') {
            return parseInt(data);
        }

        const updated = dateDiffStr(row.updated * 1000);
        const diff = dateDiffStr(data * 1000);
        return `<span title='Updated ${updated}'>${diff}</span>`;
    }

    // https://github.com/Anuken/Mindustry/blob/c339a0ecdf078391e405e58dd4c5ba2b9845bc39/core/src/mindustry/game/Gamemode.java
    function renderGamemode (data, type, row) {
        const modes = {
            0: "survival",
            1: "sandbox",
            2: "attack",
            3: "pvp",
            4: "editor"
        };
        return modes[data] || 'unknown';
    }

    function renderPlayers (data, type, row) {
        if (type == 'sort' || type == 'type') {
            return parseInt(data);
        }
        return row.limit > 0 ? data + ' / ' + row.limit : data;
    }

    function renderStatus (data, type, row) {
        if (type == 'sort' || type == 'type') {
            // small hack, abuse this for default sorting
            return data ? Number.MAX_VALUE : row.last_seen;
        }
        let status = data ? 'online' : 'offline';
        return `<img class="status" title="${status}" src="image/${status}.png">`;
    }

    function renderPing (data, type, row) {
        if (type == 'sort' || type == 'type') {
            return parseInt(data);
        }
        return data < 0 ? 'offline' : `${data}ms`;
    }

    function createdRow (row, data, dataIndex, cells) {
        $(row).addClass(`status_${data.status ? 'online' : 'offline'}`)
    }

    let table = $('#table-servers').DataTable({
        ajax: {
            url: "/api/all",
            dataSrc: (data) => {
                let servers = data.server;
                for (server of servers) {
                    server.status = server.ping != -1;
                }
                return servers;
            }
        },
        columns: [
            { data: "status", orderable: false, render: renderStatus },
            { data: "ip", orderable: true, render: renderAddress },
            { data: "host", orderable: true, render: renderString },
            { data: "description", orderable: true, render: renderString },
            { data: "map", orderable: true, render: renderString },
            { data: "players", render: renderPlayers },
            { data: "wave" },
            { data: "version" },
            { data: "vertype", orderable: true, render: renderString },
            { data: "gamemode", orderable: true, render: renderGamemode },
            { data: "ping", orderable: true, render: renderPing },
            { data: "last_seen", orderable: false, render: renderLastSeen },
        ],
        language: {
            searchPlaceholder: "Search..",
            search: ""
        },
        order: [
            [ 0, 'desc' ], // status
            [ 5, 'desc' ], // players
            [ 6, 'desc' ], // wave
        ],
        dom: '<"toolbar">frtip',
        createdRow: createdRow,
        responsive: true,
        paging: false,
        info: false
    });

    // make shift sort (multi sort) default
    $('.datatableMultiSorting th').unbind('click');
    $('.datatableMultiSorting th').click( function () {
        currentTable = $(this).closest(".datatableMultiSorting").dataTable();
        thisIndex = $(this).index();
        if (thisIndex == 0) return;
        var sortArray = [];
        $(this).siblings().addBack().each(function(index) {
            if(index==thisIndex){
                if ($(this).hasClass("sorting")){
                    sortArray.push([index,'asc']);
                }
                if ($(this).hasClass("sorting_asc")){
                    sortArray.push([index,'desc']);
                }
            } else{
                if ($(this).hasClass("sorting_asc")){
                    sortArray.push([index,'asc']);
                }
                if ($(this).hasClass("sorting_desc")){
                    sortArray.push([index,'desc']);
                }
            }
        });
        currentTable.fnSort(sortArray);
    });

    window.addServer = function (element) {
        if (event.key == 'Enter' && element.value.length > 0 ) {
            let ip = element.value;
            element.disabled = true;
            $.get('/api/server/' + ip, (e) => {
                alert(e.error || 'Server added successfully!');
                table.ajax.reload();
                element.value = '';
            })
            .fail((e) => {

            })
            .always(() => {
                element.disabled = false;
            });
        }
    }

    $("div.toolbar").html(`<div><input type="search" placeholder="Add a server" onkeydown="addServer(this)"></input></div>`);

    if (window.location.hash == '#fast') {
        setInterval(() => table.ajax.reload(), 5 * 1000);
    } else if (window.location.hash == '#dev') {
        // setInterval(() => table.ajax.reload(), 1 * 1000);
    } else {
        setInterval(() => table.ajax.reload(), 30 * 1000);
    }

    if (window.location.hash != '#dev') {
        $.fn.dataTable.ext.errMode = 'throw';
    }

})();
