console.log('(╯◣﹏◢)╯');
console.log('Oh no! An evil JS script has appeared.');

document.getElementById("page").addEventListener('touchstart', function (event) {
    touchstartX = event.changedTouches[0].screenX;
    touchstartY = event.changedTouches[0].screenY;
}, false);

document.getElementById("page").addEventListener('touchend', function (event) {
    touchendX = event.changedTouches[0].screenX;
    touchendY = event.changedTouches[0].screenY;
    handleGesture();
}, false);

document.getElementById("drawer-overlay").addEventListener('touchstart', function (event) {
    touchstartX = event.changedTouches[0].screenX;
    touchstartY = event.changedTouches[0].screenY;
}, false);

document.getElementById("drawer-overlay").addEventListener('touchend', function (event) {
    touchendX = event.changedTouches[0].screenX;
    touchendY = event.changedTouches[0].screenY;
    handleGesture();
}, false);

function handleGesture() {
    if (touchendX > (touchstartX + 50) && (touchstartY <= touchendY + 50 && touchstartY >= touchendY - 50)) {
        document.getElementById("drawer-input").checked = true;
    }
    if ((touchendX + 50) < touchstartX && document.getElementById("drawer-input").checked == true) {
        document.getElementById("drawer-input").checked = false;
    }
}