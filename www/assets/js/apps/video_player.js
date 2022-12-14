

class VideoPlayer {
    constructor(media) {
        this.media = media;


        this.player_html = "";

        if(this.media.includes("youtube")){
            var youtube_id = this.media.replace("youtube:", "");
            this.player_html = `<iframe class='video-youtube' src="https://www.youtube.com/embed/${youtube_id}" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>`
        
            // Tell sever to cache video in tmp cache
            // $.get(`/api/services/youtube/cache?id=${youtube_id}`, function( data ) {});
        
            // this.player_html = `
            // <video
            //     id="video-player"
            //     class="video-js"
            //     controls
            //     preload="auto">
            //         <source src="/tmp/youtube/${youtube_id}.m3u8" type="application/x-mpegURL"></source>
            //         <p class="vjs-no-js">
            //             To view this video please enable JavaScript, and consider upgrading to a
            //             web browser that
            //             <a href="https://videojs.com/html5-video-support/" target="_blank">
            //             supports HTML5 video
            //             </a>
            //         </p>
            // </video>
            // `;
        
        
        } else {
            this.player_html = `
                <video
                    id="video-player"
                    class="video-js"
                    controls
                    preload="auto">
                        <source src="//vjs.zencdn.net/v/oceans.mp4" type="video/mp4"></source>
                        <source src="//vjs.zencdn.net/v/oceans.webm" type="video/webm"></source>
                        <source src="//vjs.zencdn.net/v/oceans.ogv" type="video/ogg"></source>
                        <p class="vjs-no-js">
                            To view this video please enable JavaScript, and consider upgrading to a
                            web browser that
                            <a href="https://videojs.com/html5-video-support/" target="_blank">
                            supports HTML5 video
                            </a>
                        </p>
                </video>
                `;
        }
        var ref = this;
        setTimeout(function () {
            $("body").append(`<div id='video_player_container' class='video-player-container'>

                <button title="Close" class="btn btn-link video-player-exit-btn" ><i class="fas fa fa-times"></i></button>

                ${ref.player_html}

            
            </div>`);



            var options = {

                html5: {
                    vhs: {
                        overrideNative: true
                    },
                    nativeAudioTracks: false,
                    nativeVideoTracks: false
                }

            };
      
            
            ref.player = videojs('video-player', options, function onPlayerReady() {
              videojs.log('Your player is ready!');
            
              // In this context, `this` is the player that was created by Video.js.
              this.play();
            
              // How about an event listener?
              this.on('ended', function() {
                videojs.log('Awww...over so soon?!');
              });
              var ref2 = this;
              // How about an event listener?
              this.on('error', function(e) {
                ref2.reloadSourceOnError()


                ref2.pause();
                ref2.trigger("ended");
            
                ref2.reset();
            
                ref2.src(ref2.currentSrc());

              });
             
    
    
            });
            ref.player.reloadSourceOnError();


        }, 100);



      
        // var retry = 0;
        // $('video source').on('error', function() {
        //     if(retry < 4){
        //       retry++;
        //       alert('something went wrong! Retrying.. '+retry+'');
        //       $n = $(this);
        //         setTimeout(function(){
        //         $n.appendTo( $('#video') );
        //       },5000);
        //     }
        // });
     
        // player.dispose(); 
        // videojs(id);

    }

    close(){
        this.is_visible = false;
        $("#video_player_container").hide();
        $("#video_player_container").remove();
    }
}