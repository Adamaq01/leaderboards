<script lang="ts">
  import { Router, Link, Route, link, navigate } from "svelte-routing";
  import { Button } from "attractions";
  import Navbar from "./components/Navbar.svelte";
  import Home from "./views/Home.svelte";
  import Leaderboard from "./views/Leaderboard.svelte";
  import Login from "./views/Login.svelte";
  import Register from "./views/Register.svelte";
  import { token } from "./stores";

  async function logout() {
    await fetch("http://localhost:2345/auth/logout", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: "Bearer " + $token,
      },
    }).then((r) => r.json());
    token.set(null);
    navigate("/", { replace: true });
  }

  export let url = "";
</script>

{#if $token}
  <Button filled on:click={logout}>Logout</Button>
{:else}
  <a href="/login" use:link><Button filled>Login</Button></a>
{/if}
&nbsp;
<Router {url}>
  <div>
    <Route path="/">
      <Home />
    </Route>
    <Route path="/login">
      <Login />
    </Route>
    <Route path="/register">
      <Register />
    </Route>
    <Route path="/:id/*page" let:params>
      <Leaderboard id={params.id} page={params.page} />
    </Route>
  </div>
</Router>
