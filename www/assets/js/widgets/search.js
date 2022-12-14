

class SearchWidget {
    constructor() {
        this.is_visible = false;
        this.initialized = false;
        this.result_limit = 8;

        $("body").append(`<div id='search_widget_container' class='search-widget-container'>
        
        <input type="text" id="search_input" class="search-input" placeholder="Search for anything..." />
        <button onclick="search_widget.hide();" title="Close" class="btn btn-link search-exit-btn" ><i class="fas fa fa-times"></i></button>


            <div class='search-results' id='search_results'></div>

            <div class="bg-animation">
                <div id="stars"></div>
                <div id="stars2"></div>
                <div id="stars3"></div>
                <div id="stars4"></div>
            </div>

          
        </div>`);

        var ref = this;

        $("#search_input").keyup(function() {
            ref.reloadResults();
          });
        
        
    
    }

    reloadResults(){
        var ref = this;
        var html = "";

        let search_input = $("#search_input");

        $("#search_results").html(" ");
    
        $.get(`/api/services/youtube?q=${search_input.val()}`, function( data ) {

            $("#search_results").html(" ");
   
            $(data).each(function(i, obj) {
                if(i >= ref.result_limit) {
                   
                } else {
                    var video = obj["Video"];
                    if(video !== undefined) {
                        html += `<div class='video-result'>
                            
                            <img src='${video.thumbnails[3].url}' class='image'></img>
                            <p>${video.title}</p>
                            <div class="middle">
                                <button onclick="new VideoPlayer('youtube:${video.id}');" class='btn btn-primary'><i class="fas fa-play"></i></button>
                                <button onclick="downloadYoutubeVideo('${video.id}');" class='btn btn-primary'><i class="fas fa-download"></i></button>
                            </div>

                        </div>`;
                    }

                    $("#search_results").append(html);
                }
            });


            
        });
    }

    show(){
        this.is_visible = true;
        $("#search_widget_container").show();
    }

    hide(){
        this.is_visible = false;
        $("#search_widget_container").hide();
    }
}

var search_widget = new SearchWidget();

function downloadYoutubeVideo(id){
    notifications.new(`Downloading video: ${id} from YouTube...`);
    $.get(`/api/services/youtube/download?id=${id}`, function( data ) {

    });
}