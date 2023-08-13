import Camera from "./camera.js";
import QRRenderer from "./qr_renderer.js";
import { delay, StatusLabel } from "./utils.js";

async function handle_camera() {
    let reconnectAttempts = 0;
    while (true) {
        reconnectAttempts++;
        if (reconnectAttempts > 5) {
            alert("FEED ERROR!")
        }
        
        const camera = new Camera("/ws/cam", {
            image: document.getElementById("camera-feed"),
            status: new StatusLabel(document.getElementById("camera-status")),

            infoFps: document.getElementById("info-fps"),
            infoFrameCount: document.getElementById("info-frame-count")
        });

        try {
            await camera.connect()
        } catch (error) {
            await delay(2500);
            for (let i = 5; i > 0; i--) {
                camera.components.status.set(`Reconnecting in ${i} seconds`, "info")
                await delay(1000)
            }
            continue;
        }

        camera.components.status.set("Starting feed", "info")
        await delay(1000)

        try {
            await camera.poll()
        } catch {

        }

        for (let i = 5; i > 0; i--) {
            camera.components.status.set(`Reconnecting in ${i} seconds`, "info")
            await delay(1000)
        }
    }
}

async function handle_qr() {
    let reconnectAttempts = 0;
    while (true) {
        reconnectAttempts++;
        if (reconnectAttempts > 5) {
            alert("FEED ERROR!")
        }

        const qr = new QRRenderer("/ws/qr", {
            image: document.getElementById("camera-feed"),
            canvas: document.getElementById("camera-canvas")
        });

        try {
            await qr.connect()
        } catch (error) {
            console.error(error)
            await delay(2500);
            for (let i = 5; i > 0; i--) {
                await delay(1000)
            }
            continue;
        }
        await delay(1000)

        try {
            await qr.poll()
        } catch (error) {
            console.error(error)
        }

        for (let i = 5; i > 0; i--) {
            await delay(1000)
        }

    }
}

handle_qr()

handle_camera()


document.getElementById("open-button").addEventListener("click", () => {
    fetch("/action/open", {
        method: 'POST'
    })
});