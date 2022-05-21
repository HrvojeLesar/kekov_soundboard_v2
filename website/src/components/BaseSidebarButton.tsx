import {
    createStyles,
    CSSObject,
    MantineTheme,
    Tooltip,
    UnstyledButton,
} from "@mantine/core";
import { useEffect, useState } from "react";
import { Link, useMatch } from "react-router-dom";

export const baseSidebarButtonStyle = (theme: MantineTheme): CSSObject => ({
    height: "48px",
    width: "48px",
    color: theme.colors.gray[0],
    backgroundColor: theme.colors.blue[6],
    borderRadius: "50%",
    overflow: "hidden",
    display: "flex",
    textAlign: "center",
    alignItems: "center",
    justifyContent: "center",
    transition: ".2s",

    "&:hover": {
        backgroundColor: theme.colors.blue[7],
        borderRadius: "40%",
        transition: ".2s",
    },

    ":active": {
        transform: "translateY(1px)",
    },
});

export const baseSidebarButtonStyles = createStyles((theme) => ({
    baseLinkButton: baseSidebarButtonStyle(theme),
    baseLinkButtonActive: {
        ...baseSidebarButtonStyle(theme),

        backgroundColor: theme.colors.blue[7],
        borderRadius: "40%",
        transition: ".2s",
    },
}));

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
