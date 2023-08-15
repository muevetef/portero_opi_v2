import { Box, Button } from "@mui/material";
import CameraFeed from "../atomic/organisms/CameraFeed";

import { MdDoorbell } from "react-icons/md"

export default function CameraPage() {
    return (
        <Box sx={{
            display: 'grid',
            maxWidth: '100vw',
            height: '100%',
            gridTemplateAreas: `
                ". . ."
                ". cam ."
                ". controls ."
            `,
            ["@media screen and (min-width: 64em)"]: {
                gridTemplateAreas: `
                    "cam"
                    "controls"
                `
            },
        }}>
            <Box sx={{
                gridArea: 'cam',
                display: 'flex',
                justifyContent: 'center',
                alignContent: 'center',
                alignItems: 'center'
            }}>
                <CameraFeed videoSource="ws://0.0.0.0:8080/ws/cam" barSource="ws://0.0.0.0:8080/ws/qr" sx={{
                    minWidth: '200px',
                    minHeight: '200px'
                }}/>
            </Box>

            <Box sx={{
                gridArea: "controls",
                display: 'flex',
                flexDirection: 'row',
                justifyContent: 'center',
                alignContent: 'center',
                alignItems: 'center',
                marginBottom: '50px'
            }}>
                <Button variant="contained" startIcon={<MdDoorbell/>}>RING</Button>
            </Box>
        </Box>)
}