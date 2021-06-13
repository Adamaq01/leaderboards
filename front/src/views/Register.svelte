<script>
    import { FormField, TextField, Button, Modal, Dialog } from "attractions";
    import { navigate } from "svelte-routing";

    if (localStorage.TOKEN) {
        navigate("/", { replace: true });
    }

    let username = "";
    let display_name = "";
    let password = "";
    let passwordConfirm = "";

    let modalOpen = false;
    let modalTitle = "";
    let modalMessage = "";

    async function submit() {
        if (!username) {
            modalTitle = "Missing informations";
            modalMessage = "Username is missing";
            modalOpen = true;
        } else if (!display_name) {
            modalTitle = "Missing informations";
            modalMessage = "Display name is missing";
            modalOpen = true;
        } else if (!password) {
            modalTitle = "Missing informations";
            modalMessage = "Password is missing";
            modalOpen = true;
        } else if (password != passwordConfirm) {
            modalTitle = "Password missmatch";
            modalMessage = "The two passwords doesn't match";
            modalOpen = true;
        } else {
            let response = await fetch("http://localhost:2345/auth/register", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ username, display_name, password }),
            }).then((r) => r.json());
            if (response.success) {
                localStorage.TOKEN = response.token;
                navigate("/", { replace: true });
            } else {
                modalTitle = "Register failed";
                modalMessage = "Username is already taken"; // TODO response.reason
                modalOpen = true;
            }
        }
    }
</script>

<FormField name="Username" help="The name used for login" required>
    <TextField bind:value={username} />
</FormField>
<FormField
    name="Display Name"
    help="The name used that the other users will see"
    required
>
    <TextField bind:value={display_name} />
</FormField>
<FormField name="Password" help="Your password" required>
    <TextField bind:value={password} type="password" />
</FormField>
<FormField name="Password Confirmation" help="Your password" required>
    <TextField bind:value={passwordConfirm} type="password" />
</FormField>
<Button filled on:click={submit}>Register</Button>

<Modal bind:open={modalOpen} let:closeCallback>
    <Dialog title={modalTitle} {closeCallback}>{modalMessage}</Dialog>
</Modal>
