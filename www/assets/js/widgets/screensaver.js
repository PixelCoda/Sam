

class VideoPlayer {
    constructor(media) {
        this.media = media;

        $("body").append(`<div id='video_player_container' class='video-player-container'>

            <button title="Close" class="btn btn-link video-player-exit-btn" ><i class="fas fa fa-times"></i></button>

            <p>${this.media}</p>


            <div class="bg-animation">
                <div id="stars"></div>
                <div id="stars2"></div>
                <div id="stars3"></div>
                <div id="stars4"></div>
            </div>

          
        </div>`);
    }

    close(){
        this.is_visible = false;
        $("#video_player_container").hide();
        $("#video_player_container").remove();
    }
}