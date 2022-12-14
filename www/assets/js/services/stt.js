// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.


//webkitURL is deprecated but nevertheless
URL = window.URL || window.webkitURL;

var gumStream; 						//stream from getUserMedia()
var rec; 							//Recorder.js object
var input; 							//MediaStreamAudioSourceNode we'll be recording

// shim for AudioContext when it's not avb. 
var AudioContext = window.AudioContext || window.webkitAudioContext;
var audioContext //audio context to help us record


$(document).ready(function() {
	startRecording();
});


function startRecording() {
	//console.log("recordButton clicked");

    
    var constraints = { audio: true, video:false }


	navigator.mediaDevices.getUserMedia(constraints).then(function(stream) {
		//console.log("getUserMedia() success, stream created, initializing Recorder.js ...");

		audioContext = new AudioContext();

		//update the format 
		//document.getElementById("formats").innerHTML="Format: 1 channel pcm @ "+audioContext.sampleRate/1000+"kHz"

		/*  assign to gumStream for later use  */
		gumStream = stream;
		
		/* use the stream */
		input = audioContext.createMediaStreamSource(stream);

		rec = new Recorder(input,{numChannels:1})

		//start the recording process
		rec.record()

		//console.log("Recording started");

		setTimeout(stopRecording, 6500);

	}).catch(function(err) {

		//console.log(err);
	});
}

function stopRecording() {
	//console.log("stopButton clicked");

	//tell the recorder to stop the recording
	rec.stop();

	//stop microphone access
	gumStream.getAudioTracks()[0].stop();

	//create the wav blob and pass it on to createDownloadLink
	rec.exportWAV(createDownloadLink);

	startRecording();
}

function createDownloadLink(blob) {
	//name of .wav file to use during upload and download (without extendion)
	var filename = new Date().toISOString();

	var xhr=new XMLHttpRequest();
	xhr.onload=function(e) {
		if(this.readyState === 4) {
			//console.log("Server returned: ",e.target.responseText);

			const obj = JSON.parse(e.target.responseText);
			if(obj.response_type == "io"){
				speak(obj.text);
			}
		}
	};
	var fd=new FormData();
	fd.append("audio_data",blob, filename);
	xhr.open("POST","/api/services/stt",true);
	xhr.send(fd);
}