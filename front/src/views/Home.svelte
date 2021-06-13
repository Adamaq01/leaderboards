<script lang="ts">
    import { token } from "../stores";
    import { Card, Headline, Subhead, Loading } from "attractions";

    async function fetchLeaderboards() {
        return await fetch("http://localhost:2345/leaderboards", {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                Authorization: "Bearer " + $token,
            },
        }).then((r) => r.json());
    }
</script>

{#await fetchLeaderboards()}
    <Loading />
{:then value}
    {#each value as item}
        <Card outline>
            <Headline>{item.display_name}</Headline>
            <Subhead>{item.description}</Subhead>
        </Card>
    {/each}
{:catch error}
    <p>You need to login to see the leaderboards</p>
{/await}
