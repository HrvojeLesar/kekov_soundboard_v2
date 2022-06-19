import {
    createStyles,
    Group,
    LoadingOverlay,
    Paper,
    ScrollArea,
    Text,
    Title,
    UnstyledButton,
} from "@mantine/core";
import { useEffect, useState } from "react";
import { useCookies } from "react-cookie";
import { COOKIE_NAMES } from "../auth/AuthProvider";
import {
    ApiRequest,
    LOADINGOVERLAY_ZINDEX,
    primaryShade,
    PublicFile,
} from "../utils/utils";

const useStyle = createStyles(
    (theme, { isSelected }: { isSelected: boolean }) => {
        const shade = primaryShade(theme);
        return {
            paperStyle: {
                display: "flex",
                flexDirection: "column",
                overflow: "hidden",
                position: "relative",
                height: "calc(100vh - 34px)",
            },
            scollAreaStyle: {
                height: "100%",
            },
            button: {
                width: "250px",
                overflow: "hidden",
                display: "flex",
                alignItems: "center",
                transition:
                    "background-color 150ms ease, border-color 150ms ease",
                border: `1px solid ${
                    isSelected
                        ? theme.colors[theme.primaryColor][shade]
                        : theme.colorScheme === "dark"
                        ? theme.colors.dark[shade]
                        : theme.colors.gray[shade]
                }`,
                borderRadius: theme.radius.sm,
                padding: 0,
                backgroundColor: isSelected
                    ? theme.colorScheme === "dark"
                        ? theme.fn.rgba(
                              theme.colors[theme.primaryColor][shade],
                              0.3
                          )
                        : theme.fn.rgba(
                              theme.colors[theme.primaryColor][shade],
                              0.3
                          )
                    : theme.colorScheme === "dark"
                    ? theme.colors.dark[8]
                    : theme.white,

                "&:hover": {
                    transition: "150ms ease",
                    backgroundColor:
                        theme.colorScheme === "dark"
                            ? theme.fn.rgba(
                                  theme.colors[theme.primaryColor][shade],
                                  0.3
                              )
                            : theme.fn.rgba(
                                  theme.colors[theme.primaryColor][shade],
                                  0.3
                              ),
                },
            },
            unstyledButtonStyle: { width: "100%", height: "100%" },
            textStyle: {
                maxWidth: "150px",
                textOverflow: "ellipsis",
                whiteSpace: "nowrap",
                overflow: "hidden",
            },
        };
    }
);

export default function PublicFiles() {
    const [cookies] = useCookies(COOKIE_NAMES);
    const { classes } = useStyle({ isSelected: false });
    const [files, setFiles] = useState<PublicFile[]>([]);
    const [isFetching, setIsFetching] = useState(true);

    const fetchFiles = async () => {
        if (cookies.access_token) {
            try {
                const { data } = await ApiRequest.getPublicFiles(
                    cookies.access_token
                );
                setIsFetching(false);
                setFiles(data);
            } catch (e) {
                // TODO: Handle
                console.log(e);
            }
        }
    };

    useEffect(() => {
        fetchFiles();
    }, []);

    return (
        <Paper withBorder shadow="sm" p="sm" className={classes.paperStyle}>
            <Title order={3} pb="xs">
                Public sounds
            </Title>
            <LoadingOverlay
                zIndex={LOADINGOVERLAY_ZINDEX}
                visible={isFetching}
            />
            <ScrollArea className={classes.scollAreaStyle}>
                <Group>
                    {files.length > 0
                        ? !isFetching &&
                          files.map((f) => {
                              return (
                                  <Paper
                                      withBorder
                                      shadow="xs"
                                      className={classes.button}
                                  >
                                      <UnstyledButton
                                          p="sm"
                                          className={
                                              classes.unstyledButtonStyle
                                          }
                                          onClick={() => {}}
                                      >
                                          <Group spacing="sm">
                                              <Text
                                                  className={classes.textStyle}
                                                  title={f.display_name}
                                              >
                                                  {f.display_name}
                                              </Text>
                                          </Group>
                                      </UnstyledButton>
                                  </Paper>
                              );
                          })
                        : !isFetching && (
                              <Text size="xl" weight="bold">
                                  There is no public sounds.
                              </Text>
                          )}
                </Group>
            </ScrollArea>
        </Paper>
    );
}
