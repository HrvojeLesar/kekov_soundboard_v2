import { TextInput } from "@mantine/core";
import { useState } from "react";
import { TbSearch } from "react-icons/tb";

type SearchBarProps = {
    onSearch: (searchValue: string) => void;
    value?: string;
};

let filterDelay: NodeJS.Timeout;

export default function SearchBar({ onSearch, value }: SearchBarProps) {
    const [searchValue, setSearchValue] = useState(value ?? "");

    return (
        <TextInput
            value={searchValue}
            onChange={(e) => {
                const val = e.target.value;
                clearTimeout(filterDelay);
                setSearchValue(val);
                filterDelay = setTimeout(() => {
                    onSearch(val.toLowerCase());
                }, 200);
            }}
            placeholder="Search"
            rightSection={<TbSearch size={24} />}
        />
    );
}
