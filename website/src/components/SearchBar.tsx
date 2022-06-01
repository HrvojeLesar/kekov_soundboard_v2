import { TextInput } from "@mantine/core";
import { useEffect, useState } from "react";
import { Search } from "tabler-icons-react";

type SearchBarProps = {
    filterCallback: (searchValue: string) => void;
};

export default function SearchBar({ filterCallback }: SearchBarProps) {
    const [searchValue, setSearchValue] = useState("");

    useEffect(() => {
        const filterDelay = setTimeout(() => {
            filterCallback(searchValue.toLowerCase());
        }, 200);

        return () => {
            clearTimeout(filterDelay);
        };
    }, [searchValue]);

    return (
        <TextInput
            value={searchValue}
            onChange={(e) => {
                setSearchValue(e.target.value);
            }}
            placeholder="Search"
            rightSection={<Search />}
        />
    );
}
