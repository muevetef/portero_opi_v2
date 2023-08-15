import { Box, Typography } from "@mui/material"
import styles from "../../style/atomic/atoms/NavbarLink.module.scss"
import { useLocation, useNavigate } from "react-router"

type NavbarLinkProps = {
    name: string,
    link: string,
    icon: any
}

export default function NavbarLink(
    { name, link, icon }: NavbarLinkProps
) {
    const location = useLocation();
    const navigate = useNavigate();

    const active = location.pathname == link;

    return (
        <Box sx={{
            display: 'flex',
            flexDirection: 'row',
            alignItems: 'center',
            borderRadius: '8px',
            padding: '8px',
            paddingLeft: '20px',
            paddingRight: '20px',
            transitionDuration: '100ms',
            "&:hover": {
                background: '#3a8ce0',
                cursor: 'pointer'
            },
            backgroundColor: active ? '#1d6ec2' : ''
        }} onClick={() => {
            if (active) return;
            navigate(link)
        }}>
            { icon }
            <Typography sx={{ paddingLeft: '15px' }} fontSize={18}>
                {name}
            </Typography>
        </Box>
    )
}