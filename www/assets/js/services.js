// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

var paired_services = {
    spotify: false,
    lifx:  false,
    dropbox: false,
    jupiter: false,
};


$(document).ready(function() {

    
    $.get("/api/services", function( data ) {
        $(data).each(function() {
            if(this.identifier == "spotify"){
                paired_services.spotify = true;
            }
            if(this.identifier == "lifx") {
                paired_services.lifx = true;
            } 

            if(this.identifier == "dropbox") {
                paired_services.dropbox = true;
            } 
            
            if(this.identifier == "jupiter") {
                paired_services.jupiter = true;
            } 
        });

        

        if(paired_services.jupiter) {
            $("#jupiter_card_body").html(`
            <h3>Connected</h3>
            <button type="button" class="btn btn-secondary"><i class="fas fa-unlink"></i> Un-Link</button>
        `);
        } else {
           
            $("#jupiter_card_body").html(`
                <h3>Not Connected</h3>
                <button type="button" class="btn btn-primary" data-toggle="modal" data-target="#pair_jupiter_modal" aria-expanded="false" aria-controls="pair_spotify_modal" aria-label="pair_spotify_modal"><i class="fas fa-link"></i> Link</button>
            `);
        }



        if(paired_services.spotify) {
            $("#spotify_card_body").html(`
            <h3>Connected</h3>
            <button type="button" class="btn btn-secondary"><i class="fas fa-unlink"></i> Un-Link</button>
        `);
        } else {
           
            $("#spotify_card_body").html(`
                <h3>Not Connected</h3>
                <button type="button" class="btn btn-primary" data-toggle="modal" data-target="#pair_spotify_modal" aria-expanded="false" aria-controls="pair_spotify_modal" aria-label="pair_spotify_modal"><i class="fas fa-link"></i> Link</button>
            `);
        }

        if(paired_services.dropbox) {
            $("#dropbox_card_body").html(`
            <h3>Connected</h3>
            <button type="button" class="btn btn-secondary"><i class="fas fa-unlink"></i> Un-Link</button>
        `);
        } else {
           
            
            $("#dropbox_card_body").html(`
                <h3>Not Connected</h3>
                <button type="button" class="btn btn-primary" onclick="linkDropboxAccount()" aria-expanded="false"><i class="fas fa-link"></i> Link</button>
            `);
        

        }

        if(paired_services.lifx) {
            $("#lifx_card_body").html(`
            <h3>Connected</h3>
            <button type="button" class="btn btn-secondary"><i class="fas fa-unlink"></i> Un-Link</button>
        `);
        } else {
           
            $("#lifx_card_body").html(`
                <h3>Not Connected</h3>
                <button type="button" class="btn btn-primary" data-toggle="modal" data-target="#pair_lifx_modal" aria-expanded="false" aria-controls="pair_lifx_modal" aria-label="pair_lifx_modal"><i class="fas fa-link"></i> Link</button>
            `);
        }

    });
    
    
});

function linkDropboxAccount(){
    $.get("/api/services/dropbox/auth/1", function( drop_auth ) {
        Swal.fire({
            title: "", 
            html: `<a href="${drop_auth.url}" target="_blank">Click Here</a>

                <p>Then paste the code below: </p>

                <form action="/api/services/dropbox/auth/2" method="post" >
                    <input type="hidden" name="pkce" id="pkce" value="${drop_auth.pkce}" />

                    <div class="form-group">
                        <label style="color: white;">Auth Code</label>
                        <input type="text" class="form-control" name="auth_code" id="auth_code">
                    </div>

                    <button type="submit" class="btn btn-primary float-right"><i class="fas fa-link"></i> Finish</button>
                </form>
            `,  
            showConfirmButton: false
          });
    });
}

// dead
function manageSettings(service_name){
    if(service_name === "dropbox"){
        Swal.fire({
            title: "", 
            html: `
                <form action="/api/services/update_settings" method="post" >
                    <input type="hidden" name="settings" id="settings" />

                    Make Dropbox my default storage location
                    Clone stored files to dropbox
                    <button type="submit" class="btn btn-primary float-right"><i class="fas fa-link"></i> Finish</button>
                </form>
            `,  
            showConfirmButton: false
          });
    }
}