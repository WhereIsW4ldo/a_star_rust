document.addEventListener("DOMContentLoaded", () => {
    setInterval(async () => {
        let response = await fetch("/execute");
        if (response.status !== 200) {
            console.log("Error: " + response.status);
            return;
        }

        let json = await response.json();
        document.body.textContent = JSON.stringify(json, null, 2);
    })
});