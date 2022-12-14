// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.


$(document).ready(function() {

    $.get("/api/settings", function( data ) {
        console.log(data);

        var html = "";
        $(data).each(function() {
            html += `
            <tr>
                <td>${this.key}</td>
                <td><input type="text" value=${this.values} /></td>
            </tr>
            `;
        });
        $("#settings_table").html(html);


    });

});
