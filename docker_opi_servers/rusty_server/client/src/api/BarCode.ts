import { Point } from "./Point"

export type BarCode = {
    code: string,
    timestamp: any,
    points: Point[],
    frame_size: Point
}