// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.


var audio_tell_me_about_you = new Audio('/assets/audio/setup/tell_me_about_you.wav');
var audio_tell_me_about_install_location = new Audio('/assets/audio/setup/tell_me_about_install_location.wav');
var audio_connect_srv_things = new Audio('/assets/audio/setup/connect_srv_things.wav');


$(document).ready(function() {
        

    var played_intro_video = false;

    var canvas = document.getElementById('intro_video_canvas');
    var ctx = canvas.getContext('2d');
    var intro_video = document.getElementById('intro_video');

    // set canvas size = video size when known
    intro_video.addEventListener('loadedmetadata', function() {
      canvas.width = intro_video.videoWidth;
      canvas.height = intro_video.videoHeight;
    });

    intro_video.addEventListener('play', function() {
      var $this = this; //cache
      (function loop() {
        if (!$this.paused && !$this.ended) {
          ctx.drawImage($this, 0, 0);
          setTimeout(loop, 1000 / 30); // drawing at 30fps
        }
      })();
    }, 0);

    intro_video.addEventListener('ended', function() {
      played_intro_video = true;
      $("#intro_video_canvas").hide();
      $("#setup_step_1_card").show();
      audio_tell_me_about_you.play();
    });

    $('body').click(function(evnt) {
      if(!played_intro_video){
        intro_video.play();
      }
    });

    $('#step_1_nxt_btn').click(function(evnt) {
        $("#setup_step_1_card").hide();

        // Only play the first time the user click button
        if(audio_tell_me_about_install_location.played.length == 0){
            audio_tell_me_about_install_location.play();
        }
        $("#setup_step_2_card").show();
      });
    

    $("#step_2_prv_btn").click(function(evnt) {
        $("#setup_step_2_card").hide();
        $("#setup_step_1_card").show();
    });

    $("#step_2_nxt_btn").click(function(evnt) {
        $("#setup_step_2_card").hide();
        // Only play the first time the user click button
        if(audio_connect_srv_things.played.length == 0){
            audio_connect_srv_things.play();
        }
        $("#setup_step_3_card").show();
    });

    $("#step_3_prv_btn").click(function(evnt) {
        $("#setup_step_3_card").hide();
        $("#setup_step_2_card").show();
    });

  
  });