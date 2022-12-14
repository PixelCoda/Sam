// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

class RtspThings {
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

var rtsp_things = new RtspThings();

function getRtspThing(oid){
    return rtsp_things.things.filter(function (item) {
        return item.oid === oid;
    })[0];
}


class RtspThing {
    constructor(oid) {
        this.status = undefined;
        this.oid = oid;
        this.thing = undefined;
        this.stream_url = `/streams/${this.oid}.m3u8`;
    }
  
    init() {
        rtsp_things.add(this);
        // TODO - Fetch Thing

        this.update_html();
    }

    update_html(){
        var html = `
        <div class="card">
        <div class="card-header">
            <h4 class="card-title" style="text-align: center;">
            <i class="fa fas fa-video float-left"></i>
 
            <span style="position: absolute;top: 8px;left: 0;right: 0;width: 100%;text-align: center;">...</span>
            
            </h4>
        </div>
        
        <div class="card-body">
            <video style="width: 100%;" id="video_stream_${this.oid}" autoplay="true" controls="controls" type='application/x-mpegURL'></video>
        </div>
        </div>`;

        if($("#rtsp_"+this.oid).length === 0){
            var xhtml = "";
            xhtml = `<div class="col-md-12" id="rtsp_${this.oid}">`;
            xhtml += html;
            xhtml += "</div>";
            $("#things_container").append(xhtml);
        } else {
            $("#rtsp_"+this.oid).html(html);
            $("#rtsp_"+this.oid).find( "div" )[1].classList.remove("animate");
            $("#rtsp_"+this.oid).find( "div" )[2].classList.remove("animate");
        }

        var mod = this;

        if (Hls.isSupported()) {
            var video = document.getElementById(`video_stream_${this.oid}`);
            var hls = new Hls();
            // bind them together
            hls.attachMedia(video);
            hls.on(Hls.Events.MEDIA_ATTACHED, function () {
              console.log("video and hls.js are now bound together !");
              console.log(`/streams/${mod.oid}.m3u8`);
              hls.loadSource(`/streams/${mod.oid}.m3u8`);
              hls.on(Hls.Events.MANIFEST_PARSED, function (event, data) {
              });
            });
          }

    }

}
