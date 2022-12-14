// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.


var current_human = null;

var current_session = null;

var notifications = undefined;

$(document).ready(function() {


    toastr.options = {
        timeOut: 0,
        extendedTimeOut: 0
    };

    $.fn.modal.Constructor.prototype._enforceFocus = function() {}
    $.get("/api/current_human", function( data ) {
        current_human = data;
        $('.inject-human-name').each(function(i, obj) {
            $(obj).html(current_human.name);
        });
        

    });

    if(is_touch_enabled()){
        disableCursor();
    }

    $.get("/api/current_session", function( data ) {
        current_session = data;
        notifications = new Notifications(current_session);
        notifications.refresh();
        window.setInterval( function() {
            notifications.refreshUnseen()
        }, 5000)
    });


});


function newPopWindow(url, windowname, w, h, x, y)
{
    window.open(url, windowname, "resizable=no, toolbar=no, scrollbars=no, menubar=no, status=no, directories=no, width=" + w + ", height=" + h + ", left=" + x + ", top=" + y);
}

function is_touch_enabled() {
    return ( 'ontouchstart' in window ) ||
           ( navigator.maxTouchPoints > 0 ) ||
           ( navigator.msMaxTouchPoints > 0 );
}

function disableCursor(){
    var style = document.createElement('style');
    style.innerHTML = `* {
    cursor: none !important;
    }`;
    document.head.appendChild(style);
}

function uploadFile() {
    Swal.fire({
        title: "", 
        html: `
            <form id="new_file_form" class="user" action="/api/services/storage/files" method="post" enctype="multipart/form-data">

                <input type="file" id="file_data" name="file_data" style="display: block !important;">
        
            </form>
        `,  
        showConfirmButton: false
      });

      $(document).ready(function() {
        
        $( "#file_data" ).change(function() {

           
            $("#new_file_form").submit();


          });
    });
}