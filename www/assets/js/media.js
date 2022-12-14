// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

const rAF = window.mozRequestAnimationFrame || window.requestAnimationFrame;
let current_menu_item = 0;
let current_app_item = 0;
var focusable_app_area = document.getElementsByClassName('tab-pane active')[0].getElementsByClassName('controller-btn');

var focusable_menu_area = document.getElementsByClassName('tab-btn');








window.addEventListener('gamepadconnected', function (e) {
    updateLoop();
});

// event listener for vibration button
const btnVibration = document.querySelector('#btn-vibration');
if (btnVibration) {
    btnVibration.addEventListener('click', function (e) {
        hapticFeedback();
    });
}

function hapticFeedback() {
    navigator.getGamepads()[0].vibrationActuator.playEffect('dual-rumble', {
        startDelay: 0,
        duration: 1500,
        weakMagnitude: 1,
        strongMagnitude: 1
    });
}

function nextAppItem(index) {
    index++;
    current_app_item = index % focusable_app_area.length;
    focusable_app_area[current_app_item].focus();
}

function prevAppItem(index) {
    if(index > 0){
        index--;
        current_app_item = index % focusable_app_area.length;
        focusable_app_area[current_app_item].focus();
    } else {
        current_app_item = focusable_app_area.length - 1;
        focusable_app_area[current_app_item].focus();
    }
}


function nextMenuItem(index) {
    index++;
    current_menu_item = index % focusable_menu_area.length;
    focusable_menu_area[current_menu_item].focus();
}

function prevMenuItem(index) {
    if(index > 0){
        index--;
        current_menu_item = index % focusable_menu_area.length;
        focusable_menu_area[current_menu_item].focus();
    } else {
        current_menu_item = focusable_menu_area.length - 1;
        focusable_menu_area[current_menu_item].focus();
    }
}


function updateLoop() {
    let gp = navigator.getGamepads()[0];
    console.log(gp);
    console.log(current_menu_item);
    focusable_app_area = document.getElementsByClassName('tab-pane active')[0].getElementsByClassName('controller-btn');
    switch (true) {
        case gp.buttons[0].pressed:
            var element = document.activeElement;
            console.log(element);
            element.click();
            break;
        case gp.buttons[13].pressed || gp.axes[1] == 1:
            // Down direction
            nextMenuItem(current_menu_item);
            break;
        case gp.buttons[12].pressed || gp.axes[1] == -1:
            // UP Direction
            prevMenuItem(current_menu_item);
            break;
        case gp.buttons[15].pressed || gp.axes[0] == 1:
            // Right Direction
            nextAppItem(current_app_item);
            break;
        case gp.buttons[14].pressed || gp.axes[0] == -1:
            // Left Direction
            prevAppItem(current_app_item);
            break;
        default:
            break;
    }

    setTimeout(function () {
        rAF(updateLoop);
    }, 160);
}