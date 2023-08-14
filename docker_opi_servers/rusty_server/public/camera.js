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
                
                if (this.datastore.last_frame != undefined) {
                    window.URL.revokeObjectURL(this.datastore.last_frame)
                }

                this.datastore.last_frame = url;

                if (this.datastore.frame == undefined) {
                    this.datastore.frame = 0;
                }

                this.datastore.frame++;
                this.components.infoFrameCount.innerText = `Frame ${this.datastore.frame}`

                if (this.datastore.fpsInterval == undefined) {
                    this.datastore.fpsInterval = setInterval(() => {
                        this.components.infoFps.innerText = `${this.datastore.frame - this.datastore._fps} fps`;
                        this.datastore._fps = this.datastore.frame;
                    }, 1000)
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