// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

const queryString = window.location.search;
const urlParams = new URLSearchParams(queryString);
const oid = urlParams.get('oid');

var things = [];
$(document).ready(function() {

    
    $.get(`/api/rooms`, function( data ) {
        var rooms_nav_html = "";
        $(data).each(function() {

            var active = "";
            if(oid == this.oid){
                active = "active";
            }

            rooms_nav_html += `<li class="${active}">
                                    <a href="./rooms.html?oid=${this.oid}" class="controller-btn tab-btn">
                                    <i class="${this.icon}"></i>
                                    <p>${this.name}</p>
                                    </a>
                                </li>`;


        });
        $("#rooms_nav").prepend(rooms_nav_html);
    });
   
    $.get(`/api/rooms/${oid}/things`, function( data ) {



        $(data).each(function() {
            console.log(this);
            things.push(this);

            if(this.thing_type == "lifx") {

                var x = new LifXThing(this.oid);
                x.init();

                
            }

            // Ugly hack - use timeout to make sure lights load first :(
            setTimeout(() => { 

                if(this.thing_type == "rtsp") {
                    var x = new RtspThing(this.oid);
                    x.init();
                }

             }, 10000);
        

        });
       


   
    });
    
    
});

