// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

var humans_table = $("#humans_table");
var humans_table_body = $("#humans_table_body");
$(document).ready(function() {

    var html = "";
    $.get("/api/humans", function( data ) {
        $(data).each(function() {
            html += `
                <tr>
                    <td>${this.name}</td>
                    <td>${this.email}</td>
                    <td>${this.authorization_level}</td>
                    <td>
                        <button class='btn btn-xsm btn-primary'>
                            <i class="fa fa-pencil-alt"></i>
                        </button>
                        <button class='btn btn-xsm btn-danger'>
                            <i class="fa fa-trash"></i>
                        </button>
                    </td>
                </td>
            `;
        });
        $("#humans_table_body").html(html);
    });
    
    
});