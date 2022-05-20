export const nameToInitials = (guildName: string): string => {
    let initials = "";
    guildName.split(" ").forEach((word) => {
        if (word[0]) {
            initials = initials.concat(word[0]);
        }
    });
    return initials;
};
