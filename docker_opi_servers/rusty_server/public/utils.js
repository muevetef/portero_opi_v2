export function delay(ms) {
    return new Promise((resolve, _) => {
        setTimeout(() => {
            resolve()
        }, ms)
    })
}

export class StatusLabel {
    constructor(element) {
        this.element = element;
    }

    set(content, type) {
        this.element.style.display = "inline";
        this.element.innerText = content;

        if (type == "error") {
            this.element.style.color = "red"
        } else if (type == "info") {
            this.element.style.color = "white"
        }
    }

    hide() {
        this.element.style.display = "none"
    }
}