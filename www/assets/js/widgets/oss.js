

class OpenSofwareStoreWidget {
    constructor() {
        this.is_visible = false;
        this.initialized = false;
        this.result_limit = 8;
        this.packages = [];

        $("body").append(`<div id='oss_container' class='search-widget-container'>
        
            <input type="text" id="oss_search_input" class="search-input" placeholder="Search for anything..." />
            <button onclick="oss.hide();" title="Close" class="btn btn-link oss-exit-btn" ><i class="fas fa fa-times"></i></button>


            <div class='search-results' id='oss_search_results'></div>

            <div class="bg-animation">
                <div id="stars"></div>
                <div id="stars2"></div>
                <div id="stars3"></div>
                <div id="stars4"></div>
            </div>

          
        </div>`);

        var ref = this;

        $("#oss_search_input").keyup(function() {
            ref.reloadResults();
          });
        
        
    
    }

    reloadPackages(){
        var ref = this;
        $.get(`/api/services/osf/packages`, function( data ) {
            ref.packages = data;
        });
    }

    reloadResults(){
        var ref = this;
       

        let oss_search_input = $("#oss_search_input");

        $("#oss_search_results").html(" ");
  
        $("#oss_search_results").html(" ");

        $(this.packages).each(function(i, obj) {
            var html = "";

            if(!obj.category_tags.includes("core")){
                html += `<div class='oss-result'>
                            
                            <img src='data:image/png;base64, ${obj.icon_base64}' class='image'></img>
                            <p>${obj.name}</p>
                            <div class="middle">
                                <button class='btn btn-primary'><i class="fas fa-download"></i></button>
                            </div>

                        </div>`;

                $("#oss_search_results").append(html); 
            }
            
        });


      
    }

    show(){
        this.reloadPackages();
        this.reloadResults();
        this.is_visible = true;
        $("#oss_container").show();
    }

    hide(){
        this.is_visible = false;
        $("#oss_container").hide();
    }
}

var oss = new OpenSofwareStoreWidget();
oss.reloadPackages();