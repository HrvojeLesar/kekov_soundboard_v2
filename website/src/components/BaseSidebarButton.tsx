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
            backgroundColor: theme.colors[theme.primaryColor][(shade + 1) % 9],
            borderRadius: "40%",
            transition: ".2s",
        },

        "&:active": {
            transform: "translateY(1px)",
        },
    };
};

export const baseSidebarButtonStyles = createStyles(
    (theme, _params, getRef) => {
        const shade = primaryShade(theme);
        return {
            baseLinkButton: {
                ref: getRef("baseLinkButton"),
                ...baseSidebarButtonStyle(theme),
            },
            baseLinkButtonActive: {
                ref: getRef("baseLinkButtonActive"),
                ...baseSidebarButtonStyle(theme),

                borderRadius: "40%",
                transition: ".2s",
            },
            container: {
                ref: getRef("container"),
                width: "100%",
                justifyContent: "center",
                alignItems: "center",
                display: "flex",
                flexWrap: "nowrap",
                position: "relative",
            },
            notch: {
                backgroundColor:
                    theme.colorScheme === "dark"
                        ? theme.colors.gray[2]
                        : theme.colors.dark[5],
                position: "absolute",
                left: 0,
                width: "4px",
                height: "0px",
                borderRadius: "0px 4px 4px 0px",
                transition: ".2s",

                [`.${getRef("tooltip")}:hover + &`]: {
                    transition: ".2s",
                    height: "20px",
                },
            },

            notchActive: {
                backgroundColor:
                    theme.colorScheme === "dark"
                        ? theme.colors.gray[2]
                        : theme.colors.dark[5],
                position: "absolute",
                left: 0,
                width: "4px",
                height: "40px",
                borderRadius: "0px 4px 4px 0px",
                transition: ".2s",
            },
            tooltip: {
                ref: getRef("tooltip"),
                borderRadius: "50%",
            },
            tooltipActive: {
                ref: getRef("tooltipActive"),
                borderRadius: "40%",
            },
        };
    }
);

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
        <Box className={classes.container}>
            <Tooltip label={label} position="right" withArrow
                className={
                    isActive ? classes.tooltipActive : classes.tooltip}>
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
            <div
                className={isActive ? classes.notchActive : classes.notch}
            ></div>
        </Box>
    );
}
