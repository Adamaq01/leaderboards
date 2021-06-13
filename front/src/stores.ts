import { writable } from "svelte/store";

export const token = writable(localStorage.getItem("TOKEN") || "");

token.subscribe((val) => {
    if (val) localStorage.setItem("TOKEN", val);
    else localStorage.removeItem("TOKEN");
});
