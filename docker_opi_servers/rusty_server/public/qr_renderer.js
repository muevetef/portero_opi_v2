import { delay } from "./utils.js";

const COLORS = ["#f00", "#0f0", "#00f", "#ff0"]

class QRRenderer {
    constructor(address, components) {
        this.components = components;
        this.address = `ws://${window.location.host}${address}`;
        this.datastore = {}
    }

    async connect() {
        const ready = new Promise((ok, err) => {
            this.qr_socket = new WebSocket(this.address);

            this.qr_socket.addEventListener("open", () => {
                ok();
            })

            this.qr_socket.addEventListener("error", () => {
                err("Connection Refused")
            })

            this.qr_socket.addEventListener("close", () => {
                err("Connection Refused")
            })

            if (this.qr_socket.readyState == this.qr_socket.OPEN) {
                ok()
            }
        })

        try {
            await ready;
        } catch (err) {
            throw err;
        }
    }

    async poll() {
        this.datastore.canvas_ctx = this.components.canvas.getContext('2d')
        this.datastore.canvas_ctx.clearRect(0, 0, this.components.canvas.width, this.components.canvas.height);

        const result = new Promise((ok, err) => {
            this.qr_socket.addEventListener("error", (error) => {
                err("Socket Error")
            })

            this.qr_socket.addEventListener("close",(err) => {
                err(`Socket Closed: ${err}`)
            })

            this.qr_socket.addEventListener("message", (message) => {
                const qr = JSON.parse(message.data);

                const imgSize = this.components.image.getBoundingClientRect()

                this.components.canvas.width = imgSize.width;
                this.components.canvas.height = imgSize.height;
            
                const ctx = this.datastore.canvas_ctx;
                ctx.fillStyle = "#f00"
                ctx.lineWidth = 5;

                ctx.beginPath()

                let strokeStarted = false;

                function translate(position) {
                    const scaleX = imgSize.width / qr.frame_size.x
                    const scaleY = imgSize.height / qr.frame_size.y
                    console.log(scaleX, scaleY)
                    return {
                        x: position.x * scaleX,
                        y: position.y * scaleY
                    }
                }

                qr.points.forEach((point, i) => {
                    point = translate(point)
                    if (strokeStarted) {
                        ctx.lineTo(point.x, point.y);
                        ctx.stroke()
                    } 

                    ctx.strokeStyle = COLORS[(i % COLORS.length)]
                    ctx.fillStyle = COLORS[(i % COLORS.length)]

                    ctx.beginPath()
                    ctx.moveTo(point.x, point.y);

                    strokeStarted = true;

                    ctx.fillRect(point.x - 5, point.y - 5, 10, 10)
                })
                
                if (strokeStarted) {
                    const point = translate(qr.points[0])
                    ctx.lineTo(point.x, point.y);
                    ctx.stroke();
                }

                if (this.datastore.clearTimeout != undefined) {
                    clearTimeout(this.datastore.clearTimeout)

                    this.datastore.clearTimeout = setTimeout(() => {
                        ctx.clearRect(0, 0, this.components.canvas.width, this.components.canvas.height)
                    }, 1000)
                }
            })
        });

        while (this.qr_socket.readyState == this.qr_socket.OPEN) {
            await delay(1000)
        }

        try {
            await result;
        } catch (error) {
            this.datastore.canvas_ctx.clearRect(0, 0, this.components.canvas.width, this.components.canvas.height)
            console.log("Got error: ", error)
        }
    }
}

export default QRRenderer;