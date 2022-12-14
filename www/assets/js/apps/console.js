// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

var term_directory = "~";
var term_buffer = "";
var term_buffer_2 = "";
var term_cursor_location = 1;
var term_history = [];
var term_history_selected_line;
var term_count_spaceholders = 0;
var cursor_margin = 0;
var processing_command = false;
var computer_ip_addr = "";

$( document ).ready(function() {

	window.onkeydown = function(k){
		console.log(k)

		k.preventDefault();

		var banned_keys = [16, 17, 18, 19, 20, 33, 34, 35, 36, 45, 46, 93, 144, 145, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123];


		if(!processing_command){
			if(k.keyCode == 8){

				if(term_cursor_location == 1){
					term_buffer = term_buffer.slice(0, -1);
				} else {
					term_buffer = term_buffer.substring(0, (term_buffer.length) - term_cursor_location) + term_buffer.substring((term_buffer.length) - term_cursor_location + 1, (term_buffer.length));
				}

				// if(term_buffer.slice(-1) == " "){
				// 	term_cursor_location = term_cursor_location - 1;
				// 	cursor_margin = cursor_margin + 9;
				// 	$("#terminal__prompt--cursor").css("margin-left", cursor_margin+"px");
				// }
			}
			else if(k.keyCode == 32){
				//space
				term_buffer += " ";
				term_cursor_location = term_cursor_location - 1;
				cursor_margin = cursor_margin + 9;
				$("#terminal__prompt--cursor").css("margin-left", cursor_margin+"px");
				term_count_spaceholders += 1;
			}
			else if(k.keyCode == 37){
				//arrow Left
				if(term_cursor_location < term_buffer.length){
					term_cursor_location = term_cursor_location + 1;
					cursor_margin = cursor_margin - 9;
					$("#terminal__prompt--cursor").css("margin-left", cursor_margin+"px");
				}
			}
			else if(k.keyCode == 38){
				//arrow up
				// todo - history log
				// term_history
				// term_history_selected_line =
				if(term_history_selected_line == undefined){
					term_history_selected_line = term_history.length;
				}

				if(term_history_selected_line > 0){
					term_history_selected_line = term_history_selected_line - 1;
				}

				if(term_history.length > 0){
					if((term_history_selected_line + 1) == term_history.length){
						term_buffer_2 = term_buffer;
					}
					term_buffer = term_history[term_history_selected_line]
				}

				// if(term_history_selected_line == undefined){
				// 	term_buffer = term_buffer_2;
				// }

			}
			else if(k.keyCode == 39){
				//arrow Right
				if(term_cursor_location > 1){
					term_cursor_location = term_cursor_location - 1;
					cursor_margin = cursor_margin + 9;
					$("#terminal__prompt--cursor").css("margin-left", cursor_margin+"px");
				}
			}
			else if(k.keyCode == 40){

				if((term_history_selected_line + 1) == term_history.length){
					term_history_selected_line = undefined;
					term_buffer = term_buffer_2;
				}


				if(term_history_selected_line < term_history.length){
					term_history_selected_line = term_history_selected_line + 1;
					term_buffer = term_history[term_history_selected_line];
				} else {
					term_history_selected_line = undefined;
					term_buffer = term_buffer_2;
				}


				// if(term_history_selected_line == undefined){
				//
				// }
			}

			else if(k.keyCode == 9){
				//tab
			} else if(k.keyCode == 13){
				//enter
				processing_command = true;
				var html = $("#terminal__body").html().replace("term_host_name_body","").replace("terminal__prompt__buffer","").replace("terminal__prompt--cursor","").replace("id=\"terminal__prompt--location\"", "");
	 			$("#terminal__body").html(html);
				term_history.push(term_buffer);
				sendCommand(term_buffer);


			} else if(k.keyCode == 27){
				//escape
			} else {
				if(banned_keys.includes(k.keyCode)){

				}else{
					if(term_count_spaceholders > 0){
						for (let step = 0; step < term_count_spaceholders; step++) {
							term_cursor_location = term_cursor_location + 1;
							cursor_margin = cursor_margin - 9;
							$("#terminal__prompt--cursor").css("margin-left", cursor_margin+"px");
							term_count_spaceholders = term_count_spaceholders - 1;
						}
					}
					var other_keys = ["/", "\\", "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "-", "_", "+", "=", "{", "}", "[", "]", ":", ";", "'", "\"", "<", ",", ">", ".", "?", "`", "~"]
					if(k.key.match(/^[a-zA-Z1-9 ]+$/) || other_keys.includes(k.key)){

						if(term_cursor_location == 1){
							term_buffer += k.key;
						} else {
							var yyy = (term_buffer.length) - term_cursor_location;
							yyy = yyy + 1;
							term_buffer = term_buffer.substring(0, yyy) + k.key + term_buffer.substring(yyy, (term_buffer.length));
						}

					}
				}
			}
			$("#terminal__prompt__buffer").html(term_buffer);
		}

	};



	$.get( "/api/server_data", function( data ) {

		// update page
		$.each(data.clients, function( index, value ) {
			if(value.hostname == getUrlParameter("hostname")){
				$("#term_host_name_header").text(value.hostname);
				$("#term_host_name_body").text(value.hostname);
				computer_ip_addr = value.local_ip_address;
				$.get('https://'+computer_ip_addr+':6789'+"/api/cmd?command=pwd&directory="+term_directory, function( data ) {
					console.log(data);
					$("#terminal__prompt--location").html(data);
				});
			}
		});



	});

});

function sendCommand(cmd){
	var cmd_parts = cmd.split(" ");
	if(cmd_parts[0] == "cd"){
		if (cmd_parts[1].length > 0) {
			if(cmd_parts[1].charAt(0) == "/"){
				term_directory = cmd_parts[1];
			} else {
				if(term_directory == "/"){
					term_directory = term_directory + cmd_parts[1];
				} else {
					term_directory = term_directory + "/" + cmd_parts[1];
				}
			}
		}
	}

	$.get("/api/io?input="+cmd, function( data ) {
		speak(data.text).then(function () {
			$("#terminal__body").append("<span id='terminal__prompt--sam'>Sam<span></span>:</span> "+data.text.replaceAll("\n", "<br/>"));

			term_buffer = "";
			term_cursor_location = 1;
			cursor_margin = 0;
			processing_command = false;
	
			var html = "<div id='terminal__prompt'>\
						  <span id='terminal__prompt--user'>Caleb@<span id='term_host_name_body'></span>:</span>\
						  <span class='terminal__prompt--location' id='terminal__prompt--location'>~</span>\
						  <span id='terminal__prompt--bling'>$<span id='terminal__prompt__buffer'></span></span>\
						  <span id='terminal__prompt--cursor'></span>\
						</div>";
			$("#terminal__body").append(html);
	
	
			var objDiv = document.getElementById("terminal__body");
			objDiv.scrollTop = objDiv.scrollHeight;
		});


	
	});
}