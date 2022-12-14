// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.


function speak(text){
    return fetch(`/api/services/tts?text=${encodeURIComponent(text)}`).then(function (res) {
        console.log(res);    
        if (!res.ok) throw Error(res.statusText)
        return res.blob()
    }).then(function (blob) {
        console.log(blob);
        var audio = new Audio(URL.createObjectURL(blob));
        return audio.play();
    }).catch(function (err) {
       console.log(err);
    });
}


