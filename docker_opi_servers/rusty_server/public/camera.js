import { delay } from "./utils.js";

class Camera {
    constructor(address, components) {
        this.components = components;
        this.address = `ws://${window.location.host}${address}`;

        this.components.status.set("Connecting...", "info")
        this.components.image.style.display = "none"
        this.datastore = {}
    }

    async connect() {
        const ready = new Promise((ok, err) => {
            this.cam_socket = new WebSocket(this.address);

            this.cam_socket.addEventListener("open", () => {
                ok();
            })

            this.cam_socket.addEventListener("error", () => {
                err("Connection Refused")
            })

            this.cam_socket.addEventListener("close", () => {
                err("Connection Refused")
            })

            if (this.cam_socket.readyState == this.cam_socket.OPEN) {
                ok()
            }
        })

        try {
            await ready;
            this.components.status.set("Connected, receiving...", "info")
        } catch (err) {
            this.components.status.set(`Error connecting: ${err}`, "error");
            throw err;
        }
    }

    async poll() {
        this.components.status.hide();
        this.components. image.style.display = "inline-block"
        const result = new Promise((ok, err) => {
            this.cam_socket.addEventListener("error", (error) => {
                this.components.status.set(`${error}`, "error")
                this.components.image.style.display = "none";
                err("Socket Error")
            })

            this.cam_socket.addEventListener("close",(err) => {
                this.components.image.style.display = "none";
                this.components.status.set("Socket closed", "error")
                err(`Socket Closed: ${err}`)
            })

            this.cam_socket.addEventListener("message", (message) => {
                const url = window.URL.createObjectURL(message.data);
                this.components.image.src = url;

                const imgSize = this.components.image.getBoundingClientRect()
                // remove this shit and use dat attributes to store the original size, and letting the browser itself calculate the size. 
                
                if (imgSize.width > 20 && imgSize.width > 900) {
                    this.components.image.width -= 100;
                } else if (imgSize.width > 20 && imgSize.width < 200) {
                    this.components.image.width += 100;
                }
            })
        });

        while (this.cam_socket.readyState == this.cam_socket.OPEN) {
            await delay(1000)
        }

        try {
            await result;
        } catch (error) {
            this.components.image.style.display = "none";
            this.components.status.set(`Error! ${error}`, "error");
            console.log("Got error: ", error)
        }
    }
}

export default Camera;