// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

let config = {
    baseUrl: "ws://127.0.0.1:1780"
};

var things = [];
$(document).ready(function() {

   
    $.get("/api/things", function( data ) {



        $(data).each(function() {
            things.push(this);

            if(this.thing_type == "lifx") {
                // init_lifx_thing_group(this);
                // init_lifx_thing(this);
                var x = new LifXThing(this.oid);
                x.init();

                // TODO - Fix Groups

                
            }

        });
       


   
    });
    
    
});

