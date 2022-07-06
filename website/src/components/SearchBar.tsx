import { TextInput } from "@mantine/core";
import { useEffect, useRef, useState } from "react";
import { TbSearch } from "react-icons/tb";

type SearchBarProps = {
    onSearch: (searchValue: string) => void;
    value?: string;
};

export default function SearchBar({ onSearch, value }: SearchBarProps) {
    const [searchValue, setSearchValue] = useState(value ?? "");
    const isMounted = useRef(false);

    useEffect(() => {
        let filterDelay: NodeJS.Timeout;
        if (isMounted.current) {
            filterDelay = setTimeout(() => {
                onSearch(searchValue.toLowerCase());
            }, 200);
        } else {
            isMounted.current = true;
        }

        return () => {
            clearTimeout(filterDelay);
        };
    }, [searchValue, onSearch]);

    return (
        <TextInput
            value={searchValue}
            onChange={(e) => {
                setSearchValue(e.target.value);
            }}
            placeholder="Search"
            rightSection={<TbSearch size={24} />}
        />
    );
}
