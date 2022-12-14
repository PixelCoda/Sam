

class ObservationDeck {
    constructor() {
        this.observations = [];
        this.is_open = false;
    }

    refresh(page=0){

        if(page === 0){
            this.observations = [];
        }

        var ref = this;
        $.get("/api/observations?skip="+page, function( data ) {
            if(data[0] !== undefined){
                ref.observations.push(data[0]);
                if(ref.is_open){
                    $("#observations_container").append(ref.genObjHtml(data[0]));
                }
                ref.refresh(page+1);
            }
        });
    }


    open() {
        this.is_open = true;
        var ref = this;
        if(ref.observations.length > 0) {
            $("body").append(`<div id='observations_container' class='observations-container'><button onclick="observation_deck.close()" title="Close" class="btn btn-link observations-exit-btn" ><i class="fas fa fa-times"></i></button>
                ${ref.genHtml()}
            </div>`);
        } else {
            $("body").append(`<div id='observations_container' class='observations-container'><button onclick="observation_deck.close()" title="Close" class="btn btn-link observations-exit-btn" ><i class="fas fa fa-times"></i></button>
                <p>Loading observations...</p>
                ${ref.genHtml()}
            </div>`);
        }
    }

    genHtml(){
        var ref = this;
        var html = "";

        $(this.observations).each(function(i, obj) {
            html += ref.genObjHtml(obj);
        });

        return html;
    
    }


    genObjHtml(obj){
        var html = "";

        html += `<div class="observation-item">`

        html+=`<small>${obj.timestamp}</small><br/><br/>`;


        


        html+=`<video style="width: 100%; height: 200px; padding: 20px;" preload="none" controls>
            <source src="/api/observations/vwav/${obj.oid}" type="video/mp4">
            Your browser does not support the video tag.
        </video>`;


        html += `<div class="row">`;

        html += `<div class="col-md-6">`;

        html+=`<p>HUMANS:<table class='table'>`;
                    
        $(obj.observation_humans).each(function(ih, human) {
            html += `<tr>
                        <td>${human.name}</td>
                    </tr>`;
        });
        html+=`</table></p>`;

        html += `</div>`;

        html += `<div class="col-md-6">`;

        html+=`<p>NOTES:<table class='table'>`;
        
        $(obj.observation_notes).each(function(ih, obj2) {
            html += `<tr>
                        <td>${obj2}</td>
                    </tr>`;
        });
                    
        
        html+=`</table></p>`;

        html += `</div>`;
        html += `</div>`;
                    
        html+=`</div>`;

        return html;
    
    }

    close(){
        this.is_open = false;
        $("#observations_container").hide();
        $("#observations_container").remove();
    }


}

var observation_deck = new ObservationDeck();
observation_deck.refresh();