import { Box, createStyles, Paper, Title } from "@mantine/core";
import { API_URL, FilesRoute } from "../api/ApiRoutes";
import { SoundFile } from "../utils/utils";

const useStyle = createStyles((_theme) => {
    return {
        paperStyle: {
            width: "100%",
        },
        previewAudioStyle: {
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
        },
    };
});

type SoundPreviewProps = {
    selectedFile: SoundFile | undefined;
};

export default function SoundPreview({ selectedFile }: SoundPreviewProps) {
    const { classes } = useStyle();
    return (
        <Paper withBorder shadow="sm" p="sm" className={classes.paperStyle}>
            <Title order={3} pb="xs">
                Preview
            </Title>
            <Box className={classes.previewAudioStyle}>
                <audio
                    controls
                    src={`${API_URL}${FilesRoute.getPreview}${selectedFile?.owner}/${selectedFile?.id}`}
                />
            </Box>
        </Paper>
    );
}
