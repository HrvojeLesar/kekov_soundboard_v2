import { Paper, ScrollArea, Text } from "@mantine/core";

export default function Queue() {
    return (
        <Paper
            withBorder
            shadow="sm"
            style={{
                height: "300px",
                width: "100%",
                display: "flex",
                flexDirection: "column",
            }}
        >
            <Text>Queue</Text>
            <ScrollArea>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
                <Text>Scroll</Text>
            </ScrollArea>
        </Paper>
    );
}
