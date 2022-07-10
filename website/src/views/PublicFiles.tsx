import {
    Box,
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
import { useCallback, useEffect, useState } from "react";
import { useCookies } from "react-cookie";
import { useSearchParams } from "react-router-dom";
import { COOKIE_NAMES } from "../auth/AuthProvider";
import SearchBar from "../components/SearchBar";
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
                border: `1px solid ${isSelected
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
    return !Number.isNaN(page) ? page : 1;
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

    const paramsPage = searchParams.get("page");
    const paramsLimit = searchParams.get("limit");
    const paramsSearchQuery = searchParams.get("search_query");

    const handleSearch = useCallback((search: string) => {
        searchParams.set("page", "1");
        searchParams.set("search_query", search);
        setSearchParams(searchParams);
    }, [searchParams, setSearchParams]);

    useEffect(() => {
        const fetchFiles = async () => {
            if (cookies.access_token) {
                try {
                    abortController = new AbortController();
                    const { data } = await ApiRequest.getPublicFiles(
                        paramsPage,
                        paramsLimit,
                        paramsSearchQuery,
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
        abortController?.abort();
        setIsFetching(true);
        fetchFiles();
        setSelectedFile(undefined);
    }, [
        cookies.access_token,
        paramsPage,
        paramsLimit,
        paramsSearchQuery
    ]);

    useEffect(() => {
        setTotal((old) => {
            if (publicFiles === undefined) {
                return old;
            }
            const paramsNumCalc = Number(paramsLimit ?? "NaN");
            let limit = Number.isNaN(paramsNumCalc)
                ? publicFiles
                    ? publicFiles.max
                    : 50
                : paramsNumCalc > publicFiles.max
                    ? publicFiles.max
                    : paramsNumCalc;
            return Math.ceil(publicFiles?.count / limit);
        });
    }, [publicFiles, paramsLimit]);

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
                    <Box py="sm">
                        <SearchBar
                            onSearch={handleSearch}
                            value={paramsSearchQuery ?? ""}
                        />
                    </Box>
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
                                        There are no sounds to display.
                                    </Text>
                                )}
                        </Group>
                    </ScrollArea>
                    <Center>
                        <Pagination
                            page={getPageNumber(paramsPage)}
                            onChange={(page) => {
                                searchParams.set("page", page.toString());
                                setSearchParams(searchParams);
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
