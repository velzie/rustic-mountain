import init, { start, render_screen, next_tick, set_btn } from "./pkg/rustic_mountain_web.js"
const fpsInterval = 1000 / 30;
var then = 0;

var canvas;
var ctx;
var stop = false;

const palette = [
    [0, 0, 0],
    [29, 43, 83],
    [126, 37, 83],
    [0, 135, 81],
    [171, 82, 54],
    [95, 87, 79],
    [194, 195, 199],
    [255, 241, 232],
    [255, 0, 77],
    [255, 163, 0],
    [255, 236, 85],
    [0, 228, 54],
    [41, 173, 255],
    [131, 118, 156],
    [255, 119, 168],
    [255, 204, 170]
]

const keypairs = {
    37: 0,
    39: 1,
    38: 2,
    40: 3,
    90: 4,
    88: 5,
}
// var resmultiplier;

window.onload = () => {
    init().then(() => {
        // initialize canvas
        canvas = document.getElementById("canvas")
        canvas.width = 128;
        canvas.height = 128;
        if (canvas.getContext) {
            ctx = canvas.getContext("2d")
        }
        start("111111111111111111111111111111111111111111111111111111111111111111", "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", "")
        window.requestAnimationFrame(doupdate);
        window.requestAnimationFrame(draw);
    })

    window.onkeydown = (e) => {
        console.log(e.keyCode);
        let btn = keypairs[e.keyCode];
        if (btn != null) {
            set_btn(btn, true)
        }
    }
    window.onkeyup = (e) => {
        let btn = keypairs[e.keyCode];
        if (btn != null) {
            set_btn(btn, false)
        }
    }
}

function doupdate() {
    if (!stop) {
        let now = Date.now();
        let elapsed = now - then;

        window.requestAnimationFrame(doupdate);

        if (elapsed > fpsInterval) {

            then = now - (elapsed % fpsInterval);
            update()
        }
    }
}



function draw() {
    let array = render_screen();
    // console.log(array)
    let imageData = ctx.createImageData(128, 128);
    for (let i = 0; i < imageData.data.length; i += 4) {
        // console.log(array[i / 4])
        let col = palette[array[i / 4]];

        // Modify pixel data
        imageData.data[i + 0] = col[0];
        imageData.data[i + 1] = col[1];
        imageData.data[i + 2] = col[2];
        imageData.data[i + 3] = 255;
    }

    ctx.putImageData(imageData, 0, 0);
    if (!stop) {
        window.requestAnimationFrame(draw);
    }
}
function update() {
    console.log("updating");
    try {
        next_tick()
    } catch (e) {
        stop = true;
        throw e;
    }
}

// var stop = false;
// var frameCount = 0;
// var $results = $("#results");
// var fps, fpsInterval, startTime, now, then, elapsed;

// startAnimating(5);

// function startAnimating(fps) {
//     fpsInterval = 1000 / fps;
//     then = Date.now();
//     startTime = then;
//     console.log(startTime);
//     animate();
// }


// function animate() {

//     // stop
//     if (stop) {
//         return;
//     }

//     // request another frame

//     requestAnimationFrame(animate);

//     // calc elapsed time since last loop

//     now = Date.now();
//     elapsed = now - then;

//     // if enough time has elapsed, draw the next frame

//     if (elapsed > fpsInterval) {

//         // Get ready for next frame by setting then=now, but...
//         // Also, adjust for fpsInterval not being multiple of 16.67
//         then = now - (elapsed % fpsInterval);

//         // draw stuff here


//         // TESTING...Report #seconds since start and achieved fps.
//         var sinceStart = now - startTime;
//         var currentFps = Math.round(1000 / (sinceStart / ++frameCount) * 100) / 100;
//         $results.text("Elapsed time= " + Math.round(sinceStart / 1000 * 100) / 100 + " secs @ " + currentFps + " fps.");

//     }
// }