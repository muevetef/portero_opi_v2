import { Paper, SxProps, Theme } from "@mui/material"
import { useEffect, useMemo, useRef } from "react"
import { Rings } from "react-loader-spinner";
import useWebSocket from "react-use-websocket";
import { BarCode } from "../../api/BarCode";

type CameraFeedProps = {
    videoSource: string,
    barSource: string,
    className?: string | undefined,
    sx?: SxProps<Theme> | undefined
}

const COLORS = ["#f00", "#0f0", "#00f", "#ff0"]

export default function CameraFeed(
    { videoSource, barSource, className, sx }: CameraFeedProps
) {
    const { lastMessage: lastFrame, readyState: videoReady } = useWebSocket(videoSource, {
        retryOnError: true,
        reconnectInterval: 5,
        reconnectAttempts: Number.MAX_VALUE
    });
    const { lastMessage: lastBarCode, readyState: barSockReady } = useWebSocket(barSource, {
        retryOnError: true,
        reconnectInterval: 5,
        reconnectAttempts: Number.MAX_VALUE
    });
    
    const imageRef = useRef<HTMLImageElement>(null);
    const canvasRef = useRef<HTMLCanvasElement>(null)

    const canvasCtx = useMemo(() => {
        console.log("regenerating context 2d")
        const ctx = canvasRef.current?.getContext("2d")
        return ctx;
    }, [canvasRef])

    const loading = !videoReady || !barSockReady

    const imageSize = useMemo(() => {
        if (!imageRef.current) {
            return { width: 0, height: 0 }
        }

        return imageRef.current.getBoundingClientRect()
    }, [imageRef])

    const barCodeScale = useMemo(() => {
        if (!lastBarCode) return undefined;
        if (!imageSize) return undefined;

        const barData = JSON.parse(lastBarCode.data) as BarCode;

        const scaleX = imageSize.width / barData.frame_size.x
        const scaleY = imageSize.height / barData.frame_size.y

        return {
            x: scaleX,
            y: scaleY
        }
    }, [imageSize, lastBarCode])

    useEffect(() => {
        const last = imageRef.current?.src;
        
        if (last) {
            window.URL.revokeObjectURL(last);
        }

        if (!lastFrame) {
            return;
        }

        const frameData = lastFrame.data;

        const frame = window.URL.createObjectURL(frameData);

        if (imageRef.current) {
            imageRef.current.src = frame;
        }

    }, [lastFrame, imageRef])

    useEffect(() => {
        const clearTimeoutId = setTimeout(() => {
            //canvasCtx?.clearRect(0, 0, canvasRef.current?.width ?? 0, canvasRef.current?.height ?? 0)
        }, 500)

        if (!canvasCtx) return;   
        if (!barCodeScale) return;
        
        const barData = JSON.parse(lastBarCode?.data) as BarCode;

        if (!barData) return;

        canvasCtx.fillStyle = "#f00";
        canvasCtx.lineWidth = 5;
        canvasCtx.beginPath()
        let strokeStarted = false;

        barData.points.forEach((point, i) => {
            point = {
                x: point.x * barCodeScale.x,
                y: point.y * barCodeScale.y
            }

            if (strokeStarted) {
                canvasCtx.lineTo(point.x, point.y);
                canvasCtx.stroke()
            } 

            canvasCtx.strokeStyle = COLORS[(i % COLORS.length)]
            canvasCtx.fillStyle = COLORS[(i % COLORS.length)]

            canvasCtx.beginPath()
            canvasCtx.moveTo(point.x, point.y);

            strokeStarted = true;

            canvasCtx.fillRect(point.x - 5, point.y - 5, 10, 10)
        })
        
        if (strokeStarted) {
            const point = {
                x: barData.points[0].x * barCodeScale.x,
                y: barData.points[0].y * barCodeScale.y
            }
            canvasCtx.lineTo(point.x, point.y);
            canvasCtx.stroke();
        }

        return () => {
            clearTimeout(clearTimeoutId)
        }

    }, [lastBarCode, canvasCtx, barCodeScale])

    useEffect(() => {
        if (!canvasRef.current) return;

        canvasRef.current.width = imageSize.width;
        canvasRef.current.height = imageSize.height;

        canvasRef.current.style.width = imageSize.width + "px";
        canvasRef.current.style.height = imageSize.height + "px";
    }, [canvasRef, imageSize, lastBarCode])

    return (
        <Paper elevation={10} sx={{
            ...sx,
            display: 'flex',
            justifyContent: 'center',
            alignItems: 'center',
            overflow: 'hidden',
            borderRadius: '10px',
        }} className={className}>
            { loading && <Rings
                height="80"
                width="80"
                color="#808080"
                radius="6"
                visible
                ariaLabel="rings-loading"
                />
            }

            { !loading && <img ref={imageRef} style={{
                maxHeight: '800px',
                maxWidth: '1160px'
            }}/> }

            { !loading && <canvas ref={canvasRef} style={{
                position: 'absolute',
                zIndex: 99,
            }}/> }
        </Paper>
    )
}