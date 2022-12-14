// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.


class LifXThings {
    constructor() {
        this.things = undefined;
    }

    add(thing) {



        if(this.things == undefined){
            this.things = [];
        } else {
            
        }
        this.remove(thing.oid);
        this.things.push(thing);

    }

    remove(oid) {
        var i = 0;
        while (i < this.things.length) {
          if (this.things[i].oid === oid) {
            this.things.splice(i, 1);
          } else {
            ++i;
          }
        }
      }

}

var lifx_things = new LifXThings();

function getLifxThing(oid){
    return lifx_things.things.filter(function (item) {
        return item.oid === oid;
    })[0];
}


class LifXThing {
    constructor(oid, group_mode = false) {
        this.status = undefined;
        this.group_mode = group_mode;
        this.oid = oid;
        this.thing = undefined;
        this.public_obj = undefined;
        this.private_obj = undefined;
    }
  
    init(expectations = undefined) {
        lifx_things.add(this);
        this.status = "initialization";
        this.update_html();
        var modelref = this;

        $.get("/api/services/lifx/public/list", function( lifx_live_data ) {

            if(modelref.group_mode){
               
                $(lifx_live_data).each(function() {
                    if(modelref.oid == this.group.name){
                        modelref.public_obj = this;
                        lifx_things.add(modelref);
                        if(modelref.status == "initialization"){
                            modelref.status = "1";
                        }
                        if(modelref.status == "1"){
                            modelref.status = "2";
                        }
                        
                        if(expectations !== undefined){
                            if(expectations == "power:on"){
                                if(this.power == "off"){
                                    setTimeout(modelref.init(expectations), 100);
                                    return modelref;
                                }
                            }
                            if(expectations == "power:off"){
                                if(this.power == "on"){
                                    setTimeout(modelref.init(expectations), 100);
                                    return modelref;
                                }
                            }
                        }

        
                        if(this.connected == false){
                            modelref.private_obj = this;
                            modelref.status = "2";
                        }
                        

             

                        modelref.update_html();


                        return modelref;
                    }
                });
  
            } else {
                $.get("/api/things", function( data ) {
                    $(data).each(function() {
                        if(this.oid == modelref.oid){
                            var thing = this;
                            modelref.thing = this;
                            $(lifx_live_data).each(function() {
                                if(thing.name == this.label){
                                    modelref.public_obj = this;
                                    lifx_things.add(modelref);
                                    if(modelref.status == "initialization"){
                                        modelref.status = "1";
                                    }
                                    if(modelref.status == "1"){
                                        modelref.status = "2";
                                    }
                                  
                                    if(expectations !== undefined){
                                        if(expectations == "power:on"){
                                            if(this.power == "off"){
                                                setTimeout(modelref.init(expectations), 100);
                                                return modelref;
                                            }
                                        }
                                        if(expectations == "power:off"){
                                            if(this.power == "on"){
                                                setTimeout(modelref.init(expectations), 100);
                                                return modelref;
                                            }
                                        }
                                    }
    
                                    if(this.connected == false){
                                        modelref.private_obj = this;
                                        modelref.status = "2";
                                    }
    
                                    if(this.group !== undefined && $(`#lifx_${this.group.name}`).length === 0){
                                        var x = new LifXThing(this.group.name, true);
                                        x.init();
                                    }
    
    
                                    modelref.update_html();
    
    
                                    return modelref;
                                }
                            });
                            return modelref;
                        }
                    });
                });
            }



        });
        $.get("/api/services/lifx/private/list", function( lifx_live_data ) {

            if(modelref.group_mode){
               
                $(lifx_live_data).each(function() {
                    if(modelref.oid == this.group.name){
                        modelref.private_obj = this;
                        lifx_things.add(modelref);
                        if(modelref.status == "initialization"){
                            modelref.status = "1";
                        }
                        if(modelref.status == "1"){
                            modelref.status = "2";
                        }
                        
                        if(expectations !== undefined){
                            if(expectations == "power:on"){
                                if(this.power == "off"){
                                    setTimeout(modelref.init(expectations), 100);
                                    return modelref;
                                }
                            }
                            if(expectations == "power:off"){
                                if(this.power == "on"){
                                    setTimeout(modelref.init(expectations), 100);
                                    return modelref;
                                }
                            }
                        }

            

                        modelref.update_html();


                        return modelref;
                    }
                });
  
            } else {
                $.get("/api/things", function( data ) {
                    $(data).each(function() {
                        if(this.oid == modelref.oid){
                            var thing = this;
                            modelref.thing = this;
                            $(lifx_live_data).each(function() {
                                if(thing.name == this.label){
                                    modelref.private_obj = this;
                                    lifx_things.add(modelref);
                                    if(modelref.status == "initialization"){
                                        modelref.status = "1";
                                    }
                                    if(modelref.status == "1"){
                                        modelref.status = "2";
                                    }
                           
    
    
                                    if(expectations !== undefined){
                                        if(expectations == "power:on"){
                                            if(this.power == "off"){
                                                setTimeout(modelref.init(expectations), 100);
                                                return modelref;
                                            }
                                        }
                                        if(expectations == "power:off"){
                                            if(this.power == "on"){
                                                setTimeout(modelref.init(expectations), 100);
                                                return modelref;
                                            }
                                        }
                                    }
                                    modelref.update_html();
    
                                    return modelref;
                                }
                            });
                            return modelref;
                        }
                    });
                });
            }



        
        });
    }


    set_kelvin(){
        var kelvin = $(`#lifx_kelvin`).val();
        console.log("set_kelvin");
        if(this.group_mode){
            $.post( "/api/services/lifx/set_color", { use_public: "true", selector: `group_id:${this.public_obj.group.id}`, color: `kelvin:${kelvin}` } );
        } else {
            if(this.private_obj !== undefined){
                $.post( "/api/services/lifx/set_color", { use_public: "false", selector: `id:${this.private_obj.id}`, color: `kelvin:${kelvin}` } );
            } else if(this.public_obj !== undefined){
                $.post( "/api/services/lifx/set_color", { use_public: "true", selector: `id:${this.public_obj.id}`, color: `kelvin:${kelvin}` } );
            }
        }
        
    }

    set_color(){
        var color = $(`#lifx_colorpicker`).val();
    
        if(this.group_mode){
            $.post( "/api/services/lifx/set_color", { use_public: "true", selector: `group_id:${this.public_obj.group.id}`, color: color } );
        } else {
            $.post( "/api/services/lifx/set_color", { use_public: "true", selector: `id:${this.public_obj.id}`, color: color } );
        }
        
    }

    set_state(power){

        // Animate HTML Objects
        $("#lifx_"+this.oid).find( "div" )[1].classList.add("animate");
        $("#lifx_"+this.oid).find( "div" )[2].classList.add("animate");
        $($("#lifx_"+this.oid).find( "div" )[2]).find("button")[0].disabled = true;
        $($($("#lifx_"+this.oid).find( "div" )[2]).find("h3")[0]).html("..........");
    

 
        if(this.group_mode){
            $.post( "/api/services/lifx/set_state", { use_public: "true", selector: `group_id:${this.public_obj.group.id}`, power: power } );
        
            var mod = this;
            // re-initialize group members
            $(lifx_things.things).each(function() {
                console.log(this);
                if(this.public_obj !== undefined){
                    if(this.public_obj.group.name == mod.oid){
                        this.init(`power:${power}`);
                    }
                } else {
                    if(this.private_obj !== undefined){
                        if(this.private_obj.group.name == mod.oid){
                            this.init(`power:${power}`);
                        }
                    }
                }
              
            });
        
        } else {
            if(this.private_obj !== undefined){
                $.post( "/api/services/lifx/set_state", { use_public: "false", selector: `id:${this.private_obj.id}`, power: power } );
            } else if(this.public_obj !== undefined){
                $.post( "/api/services/lifx/set_state", { use_public: "true", selector: `id:${this.public_obj.id}`, power: power } );
            }
           
            var mod = this;
            $(lifx_things.things).each(function() {
                console.log(this);
                if(this.public_obj !== undefined){
                    if(this.public_obj.group.name == mod.public_obj.group.name){
                        if(this.group_mode){
                            this.init();
                        }
                    }
                }
            });
        }


        // Update HTML with expectations
        this.init(`power:${power}`);
        
    }

    
    update_html(){


        var html = "";

        if(this.status == "initialization" || this.status == "1"){
            html = `
            <div class="card animate">
            <div class="card-header animate">
                <h4 class="card-title" style="text-align: center;">
                <i class="bi bi-lightbulb-off float-left"></i>
     
                <span style="position: absolute;top: 8px;left: 0;right: 0;width: 100%;text-align: center;">...</span>
                
                </h4>
            </div>
            
            <div class="card-body animate">

            </div>
            </div>`;


            if($("#lifx_"+this.oid).length === 0){
                var xhtml = "";
                xhtml = `<div class="col-md-6" id="lifx_${this.oid}">`;
                xhtml += html;
                xhtml += "</div>";
                $("#things_container").append(xhtml);
            } else {
                $("#lifx_"+this.oid).find( "div" )[1].classList.add("animate");
                $("#lifx_"+this.oid).find( "div" )[2].classList.add("animate");

                if($($("#lifx_"+this.oid).find( "div" )[1]).find("button")[0] !== undefined){
                    $($("#lifx_"+this.oid).find( "div" )[1]).find("button")[0].disabled = true;
                    $($("#lifx_"+this.oid).find( "div" )[2]).find("button")[0].disabled = true;
                }

           
            }

            return false;


        }




        if(this.public_obj == undefined && this.private_obj == undefined){
            setTimeout(this.update_html(), 500);
            return false;
        }

     
        var icon = "bi bi-lightbulb-off";
        var button_html = `<button onclick="getLifxThing('${this.oid}').set_state('on')" type="button" class="btn btn-sm btn-secondary"><i class="fas fa-bolt"></i> Turn On</button>`;
        var button_color = "secondary";


        if((this.public_obj !== undefined && this.public_obj.power == "on") || ( this.private_obj !== undefined && this.private_obj.power == "on")){
            icon = "bi bi-lightbulb";
            button_color = "primary";
            button_html = `<button onclick="getLifxThing('${this.oid}').set_state('off')" type="button" class="btn btn-sm btn-primary"><i class="fas fa-power-off"></i> Turn Off</button>`;
        }

        if((this.public_obj === undefined) && ( this.private_obj === undefined)){
            button_html = `<button type="button" class="btn btn-sm btn-secondary" disabled><i class="fas fa-exclamation-triangle"></i> Offline</button>`;
        } 
    
    
                    html += `
                        <div class="card ">
                        <div class="card-header">
                            <h4 class="card-title" style="text-align: center;">
                            <i class="${icon} float-left"></i>`;

                    var title = "Unknown";
                    if(!this.group_mode){
                        title = this.thing.name;
                        html += `<small style="font-size: 10px;position: absolute;top: 28px;left: 0;right: 0;width: 100%;text-align: center;"><a href="/rooms.html?oid=${this.thing.room.oid}">${this.thing.room.name}</a></small>`;
                    } else {
                        if(this.public_obj !== undefined && this.public_obj !== null){
                            title = this.public_obj.group.name + " Group";
                        }
                        
                    }
                    
                    
                    html += `<span style="position: absolute;top: 8px;left: 0;right: 0;width: 100%;text-align: center;">${title}</span>
                            
                            <button onclick="getLifxThing('${this.oid}').settings()" class="btn btn-xsm btn-${button_color}" href="#" style="
                                right: 10px;
                                top: 10px;
                                position: absolute;
                            "><i class="fa fa-cog"></i></button>
    
                            `;
    
    
                    html += `</h4>
                        </div>
                        
                        <div class="card-body">
                            <center>
                                ${button_html}
                            </center>
                        </div>
                        `;
    
                   
    
                    $($($("#lifx_"+this.oid).find( "div" )[2]).find("h3")[0]).html(this.power);
                       
                    html+= "</div>";
                        
    
            if($("#lifx_"+this.oid).length === 0){
                var xhtml = "";
                xhtml = `<div class="col-md-6" id="lifx_${this.oid}">`;
                xhtml += html;
                xhtml += "</div>";
                $("#things_container").append(xhtml);
            } else {
                $("#lifx_"+this.oid).html(html);
                $("#lifx_"+this.oid).find( "div" )[1].classList.remove("animate");
                $("#lifx_"+this.oid).find( "div" )[2].classList.remove("animate");
                $($("#lifx_"+this.oid).find( "div" )[1]).find("button")[0].disabled = false;
                $($("#lifx_"+this.oid).find( "div" )[2]).find("button")[0].disabled = false;
                
            }
    
   
    }
    

    settings(){
        var html = "";
    

        if(this.private_obj !== undefined){
            html += `<input onchange="getLifxThing('${this.oid}').set_kelvin()" type="range" min="${this.private_obj.product.capabilities.min_kelvin}" max="${this.private_obj.product.capabilities.max_kelvin}" value="${this.private_obj.color.kelvin}" class="slider" id="lifx_kelvin">`;
        } else if(this.public_obj !== undefined){
            html += `<input onchange="getLifxThing('${this.oid}').set_kelvin()" type="range" min="${this.public_obj.product.capabilities.min_kelvin}" max="${this.public_obj.product.capabilities.max_kelvin}" value="${this.public_obj.color.kelvin}" class="slider" id="lifx_kelvin">`;
        }
      
        html += `<input onchange="getLifxThing('${this.oid}').set_color()" type="color" id="lifx_colorpicker" value="#0000ff">`;
        // Add Color Wheel to html

        console.log(this);

            

            var name = ``;


            if(this.private_obj !== undefined){
                name = this.private_obj.label;
            } else if(this.public_obj !== undefined){
                name = this.public_obj.label;
            }

            if(this.group_mode){
                if(this.private_obj !== undefined){
                    name = `${this.private_obj.group.name} Group`;
                } else if(this.public_obj !== undefined){
                    name = `${this.public_obj.group.name} Group`;
                }

            }
            Swal.fire({
                title: `Settings for ${name}`, 
                html: html,  
            });
        
            return false;
        

    
   
    
            
    
    
    
    
    }
    
    









  }






