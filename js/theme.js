if (
    !window.matchMedia("(prefers-color-scheme: dark)").matches &&
    !(window.location.href === "https://stephenpower.ie/html/light/home.html" || window.location.href === "https://stephenpower.ie/html/light/home.html")
) {
    window.location.href = "https://stephenpower.ie/html/light/home.html";
}