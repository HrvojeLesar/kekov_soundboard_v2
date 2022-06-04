import {
    Box,
    createStyles,
    CSSObject,
    MantineTheme,
    Tooltip,
    UnstyledButton,
} from "@mantine/core";
import { useEffect, useState } from "react";
import { Link, useMatch } from "react-router-dom";
import { primaryShade } from "../utils/utils";

export const baseSidebarButtonStyle = (theme: MantineTheme): CSSObject => {
    const shade = primaryShade(theme);
    return {
        height: "48px",
        width: "48px",
        color: theme.colors.gray[0],
        backgroundColor: theme.colors[theme.primaryColor][shade],
        borderRadius: "50%",
        overflow: "hidden",
        display: "flex",
        textAlign: "center",
        alignItems: "center",
        justifyContent: "center",
        transition: ".2s",

        "&:hover": {
            backgroundColor:
                theme.colors[theme.primaryColor][(shade + 1) % 9],
            borderRadius: "40%",
            transition: ".2s",
        },

        "&:active": {
            transform: "translateY(1px)",
        },
    };
};

export const baseSidebarButtonStyles = createStyles((theme) => {
    const shade = primaryShade(theme);
    return {
    baseLinkButton: baseSidebarButtonStyle(theme),
    baseLinkButtonActive: {
        ...baseSidebarButtonStyle(theme),

        borderRadius: "40%",
        transition: ".2s",
    },
}});

type Props = {
    children: JSX.Element;
    route: string;
    label: string;
    className?: string;
};

export default function BaseSidebarButton({ children, route, label }: Props) {
    const { classes } = baseSidebarButtonStyles();
    const match = useMatch(route);
    const [isActive, setIsActive] = useState(false);

    useEffect(() => {
        if (match != null) {
            setIsActive(true);
        } else {
            setIsActive(false);
        }
    }, [match]);

    return (
        <Tooltip label={label} position="right" withArrow>
            <UnstyledButton
                className={
                    isActive
                        ? classes.baseLinkButtonActive
                        : classes.baseLinkButton
                }
                component={Link}
                to={route}
            >
                {children}
            </UnstyledButton>
        </Tooltip>
    );
}
