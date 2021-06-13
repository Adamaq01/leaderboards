<script>
    import { FormField, TextField, Button, Modal, Dialog } from "attractions";
    import { navigate } from "svelte-routing";
    import { token } from "../stores";

    if ($token) {
        navigate("/", { replace: true });
    }

    let username = "";
    let password = "";

    let modalOpen = false;
    let modalTitle = "";
    let modalMessage = "";

    async function submit() {
        if (!username) {
            modalTitle = "Missing informations";
            modalMessage = "Username is missing";
            modalOpen = true;
        } else if (!password) {
            modalTitle = "Missing informations";
            modalMessage = "Password is missing";
            modalOpen = true;
        } else {
            let response = await fetch("http://localhost:2345/auth/login", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ username, password }),
            }).then((r) => r.json());
            if (response.success) {
                token.set(response.token);
                navigate("/", { replace: true });
            } else {
                modalTitle = "Authentication failed";
                modalMessage = ""; // TODO response.reason
                modalOpen = true;
            }
        }
    }
</script>

<FormField name="Username" help="The name used for login" required>
    <TextField bind:value={username} />
</FormField>
<FormField name="Password" help="Your password" required>
    <TextField bind:value={password} type="password" />
</FormField>
<Button filled on:click={submit}>Login</Button>

<Modal bind:open={modalOpen} let:closeCallback>
    <Dialog title={modalTitle} {closeCallback}>{modalMessage}</Dialog>
</Modal>
