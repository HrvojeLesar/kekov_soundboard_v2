import {
    Center,
    createStyles,
    Grid,
    Group,
    LoadingOverlay,
    Pagination,
    Paper,
    ScrollArea,
    Text,
    Title,
} from "@mantine/core";
import { useEffect, useState } from "react";
import { useCookies } from "react-cookie";
import { useQuery } from "react-query";
import { useLocation, useSearchParams } from "react-router-dom";
import { URLSearchParams } from "url";
import { COOKIE_NAMES } from "../auth/AuthProvider";
import ServerSelect from "../components/UserFiles/ServerSelect";
import SelectableFileContainer from "../components/UserFiles/UserFileContainer";
import {
    ApiRequest,
    LOADINGOVERLAY_ZINDEX,
    primaryShade,
    PublicFiles as PublicFilesType,
    SoundFile,
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

const getPageNumber = (initialPage: string | null) => {
    const page = Number(initialPage ?? 1);
    return page !== NaN ? page : 1;
};

let abortController: AbortController | undefined = undefined;

export default function PublicFiles() {
    const [cookies] = useCookies(COOKIE_NAMES);
    const { classes } = useStyle({ isSelected: false });
    const [selectedFile, setSelectedFile] = useState<SoundFile | undefined>(
        undefined
    );
    const [total, setTotal] = useState(1);
    const [searchParams, setSearchParams] = useSearchParams();
    const [isFetching, setIsFetching] = useState(true);
    const [publicFiles, setPublicFiles] = useState<PublicFilesType | undefined>(
        undefined
    );

    const fetchFiles = async () => {
        if (cookies.access_token) {
            try {
                abortController = new AbortController();
                const { data } = await ApiRequest.getPublicFiles(
                    searchParams.get("page"),
                    searchParams.get("limit"),
                    cookies.access_token,
                    abortController
                );
                setPublicFiles(data);
            } catch (e) {
                console.log(e);
            } finally {
                setIsFetching(false);
            }
        }
    };

    useEffect(() => {
        abortController?.abort();
        setIsFetching(true);
        fetchFiles();
        setSelectedFile(undefined);
    }, [searchParams.get("page")]);

    useEffect(() => {
        setTotal((old) => {
            if (publicFiles === undefined) {
                return old;
            }
            const paramsNumCalc = Number(searchParams.get("limit") ?? "NaN");
            let limit = Number.isNaN(paramsNumCalc)
                ? publicFiles
                    ? publicFiles.max
                    : 50
                : paramsNumCalc > publicFiles.max
                ? publicFiles.max
                : paramsNumCalc;
            return Math.ceil(publicFiles?.count / limit);
        });
    }, [publicFiles]);

    return (
        <Grid>
            <Grid.Col xs={9}>
                <Paper
                    withBorder
                    shadow="sm"
                    p="sm"
                    className={classes.paperStyle}
                >
                    <Title order={3} pb="xs">
                        Public sounds
                    </Title>
                    <LoadingOverlay
                        zIndex={LOADINGOVERLAY_ZINDEX}
                        visible={isFetching}
                    />
                    <ScrollArea mb="xs" className={classes.scollAreaStyle}>
                        <Group>
                            {publicFiles && publicFiles.files.length > 0
                                ? !isFetching &&
                                  publicFiles.files.map((f) => {
                                      return (
                                          <SelectableFileContainer
                                              key={f.id}
                                              file={f}
                                              isSelected={
                                                  selectedFile?.id === f.id
                                              }
                                              onClickCallback={(f) => {
                                                  setSelectedFile(f);
                                              }}
                                          />
                                      );
                                  })
                                : !isFetching && (
                                      <Text size="xl" weight="bold">
                                          There is no public sounds.
                                      </Text>
                                  )}
                        </Group>
                    </ScrollArea>
                    <Center>
                        <Pagination
                            page={getPageNumber(searchParams.get("page"))}
                            onChange={(page) => {
                                if (
                                    !searchParams.get("limit") ||
                                    searchParams.get("limit")?.trim() === ""
                                ) {
                                    setSearchParams({
                                        page: page.toString(),
                                    });
                                } else {
                                    setSearchParams({
                                        page: page.toString(),
                                        limit:
                                            searchParams.get("limit") ??
                                            publicFiles?.max.toString() ??
                                            "100",
                                    });
                                }
                            }}
                            total={total}
                        />
                    </Center>
                </Paper>
            </Grid.Col>
            <Grid.Col xs={3}>
                <ServerSelect file={selectedFile} />
            </Grid.Col>
        </Grid>
    );
}
