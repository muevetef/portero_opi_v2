import { Box, Paper, Typography, useMediaQuery } from "@mui/material";
import { Outlet } from "react-router";

import styles from "../style/MainLayout.module.scss"
import NavbarLink from "../atomic/atoms/NavbarLink";

import { MdCameraAlt, MdGroup } from "react-icons/md"; 

export default function MainLayout() {
    const isMobile = useMediaQuery('(max-width: 800px)')

    return (
        <Paper sx={{
            width: '100vw',
            height: '100vh',
            margin: '0',
            top: '0px',
            left: '0px',
            position: 'absolute',
            borderRadius: '0'
        }} className={`${styles.layout} ${isMobile && styles.mobile}`}>
            <Paper className={`${styles.navbar} ${isMobile && styles.mobile}`} elevation={5} sx={{
                borderRadius: '0',
            }}>
                <Typography fontSize={32}>Portero</Typography>

                <Box className={`${styles.navbarLinks} ${isMobile && styles.mobile}`}>
                    <NavbarLink name="Camera" icon={<MdCameraAlt/>} link="/feed"/>
                    <NavbarLink name="Users" icon={<MdGroup/>} link="/users"/>
                </Box>
            </Paper>
            <Paper sx={{
                width: '100%',
                height: '100%',
                margin: '0',
                overflow: 'hidden'
            }}>
                <Outlet/>
            </Paper>
        </Paper>
    )
}