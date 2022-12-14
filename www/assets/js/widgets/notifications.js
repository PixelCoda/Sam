

class Notifications {
    constructor(current_session) {
        this.current_session = current_session;
        this.notifications = [];
        this.toasted_notifications = [];
        this.is_open = false;
    }


    refreshUnseen(){
        var ref = this;
        $.get("/api/services/notifications/unseen", function( data ) {

            var reversed = data.reverse();
            $(reversed).each(function(i, obj) {
                console.log(obj.message);
                if(!obj.seen && !ref.toasted_notifications.includes(obj.oid)){

                    ref.toasted_notifications.push(obj.oid);
                    setTimeout(() => { 
                        
                        toastr.info(obj.message, '', 
                            {onclick: function() {
                                ref.markAsSeen(obj.oid);
                            }}
                        );
                    
                    }, 100);


                    
                    
                }
            });
        });
    }

    refresh(){
        var ref = this;
        $.get("/api/services/notifications", function( data ) {
            ref.notifications = data;
            if(ref.is_open){
                $("#notifications_container").html(ref.genHtml());
            }
        });


    }

    markAsSeen(oid){
        $.post("/api/services/notifications/seen", { oid: oid } );
    }

    open() {
        this.is_open = true;
        // this.refresh();
        var ref = this;
        $("body").append(`<div id='notifications_container' class='notifications-container'>

            ${ref.genHtml()}

        </div>`);
    }

    genHtml(){
        var html = "";
        html += `<button onclick="notifications.close()" title="Close" class="btn btn-link notifications-exit-btn" ><i class="fas fa fa-times"></i></button>`;
    
        $(this.notifications).each(function(i, obj) {

            html += `<div class="notification-item">
                        <p>${obj.message}</p>
                        <small>${obj.timestamp}</small>
                    </div>`;
        });

        return html;
    
    }

    close(){
        this.is_open = false;
        $("#notifications_container").hide();
        $("#notifications_container").remove();
    }


    new(message){
        $.post("/api/services/notifications", { message: message } );
    }
}

