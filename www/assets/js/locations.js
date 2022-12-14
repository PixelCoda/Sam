// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

$(document).ready(function() {

    
    $.get("/api/locations", function( location_data ) {

       
        $(location_data).each(function() {
            var location = this;


          



           
            var html = "";
            var rooms_html = "";
            $.get(`/api/locations/${location.oid}/rooms`, function( room_data ) {
  

                rooms_html = `
            
                <div class="modal fade" id="manage_rooms_for_location_${location.oid}_modal" tabindex="-1" role="dialog" aria-labelledby="exampleModalLabel" aria-hidden="true">
                    <div class="modal-dialog" role="document">
                    <div class="modal-content">
                        <div class="modal-header">
                        <h5 class="modal-title" id="exampleModalLabel">${location.name}</h5>
                        <button type="button" class="close" data-dismiss="modal" aria-label="Close">
                            <span aria-hidden="true">&times;</span>
                        </button>
                        </div>
                   
                        <div class="modal-body">`;

                        if(room_data.length == 0){
                            rooms_html += 'No rooms yet...'
                        } else {

                            rooms_html += `
                            
                            <table class="table tablesorter">
                                <thead class=" text-primary">
                                    <tr>
                                        <th>
                                            Name
                                        </th>
                                        <th class="text-center">
                                            
                                        </th>
                                    </tr>
                                </thead>
                                <tbody>`;

                            
                            

                            $(room_data).each(function() {
                     
                                rooms_html += `<tr>
                                    <td>${this.name}</td>
                                </tr>`;


                            });

                            rooms_html += `</tbody>
                            </table>`;
                        }
                        
  
                    rooms_html += `</div>
                        <div class="modal-footer">
                            <button type="button" onclick="addRoomToLocation('${location.oid}');" class="btn btn-primary float-right">Add Room</button>
                        </div>
                        
                    </div>
                    </div>
                </div>
                
                `;

                html += `
                    <div class="col-md-3" id="lifx_pxlITlL9w3Lf2Z2">
                        <div class="card ">
                            <div class="card-header">
                                <h4 class="card-title" style="text-align: center;"><i class="fas fa-lightbulb float-left"></i><span>${location.name}</span></h4>
                            </div>
                            <div class="card-body" id="spotify_card_body">
                                <p>${location.address}</p>
                                <button type="button" data-toggle="modal" data-target="#manage_rooms_for_location_${location.oid}_modal" class="btn btn-primary"><i class="fas fa-map-signs"></i> Manage Rooms</button>
                            </div>
                        </div>
                    </div>
                `;
                html += rooms_html;
                
      
                $("#locations_table_body").append(html);

            });

            
        });
        
    });
    
    
});


function addRoomToLocation(location_oid){

    Swal.fire({
        title: 'New Room',
        input: 'text',
        inputLabel: 'Name',
        showCancelButton: true,
        inputValidator: (value) => {
          if (!value) {
            return 'You need to write something!'
          }
        },
        preConfirm:function(){
            in1= $('#swal2-input').val();


            $.post(`/api/locations/${location_oid}/rooms`, { name: in1 } );


            console.log(in1) // use user input value freely
        }
      })

}