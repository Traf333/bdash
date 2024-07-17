<script lang="ts">
    import {
        Button,
        Checkbox,
        Input,
        Label,
        Modal,
        Textarea,
    } from "flowbite-svelte";
    import type { TUser, TUserResponse } from "../../types";
    import { users } from "./store";
    import { invoke } from "@tauri-apps/api/core";

    export let open: boolean = false; // modal control

    export let data: TUser = {
        name: "",
        init_data: "",
        status: false,
    };
    const submit = async () => {
        console.log(data);
        if (data.id) {
            let response = await invoke<TUser>("update_account", {
                id: data.id.id.String,
                content: data,
            });
            console.log("after update response", response);
            users.updateUser(response);
        } else {
            let response = await invoke<TUser[]>("create_account", {
                content: data,
            });

            console.log("after create response", response);
            users.addUser(response[0]);
        }
        open = false;
    };
</script>

<Modal
    bind:open
    title={data.id ? "Edit user" : "Add new user"}
    size="md"
    class="m-4"
>
    <!-- Modal body -->
    <div class="space-y-6 p-0">
        <form on:submit={submit}>
            <div class="grid grid-cols-6 gap-6">
                <Label class="col-span-6 space-y-2 sm:col-span-6">
                    <span>Name</span>
                    <Input
                        name="name"
                        placeholder="e.g. Bonnie Green"
                        bind:value={data.name}
                        required
                    />
                </Label>

                <Label class="col-span-6 space-y-2 sm:col-span-6">
                    <span>Init Data</span>
                    <Textarea
                        name="init_data"
                        placeholder="e.g. "
                        bind:value={data.init_data}
                        required
                    />
                </Label>
                <Label class="col-span-6 space-y-2 sm:col-span-3">
                    <Checkbox name="status" bind:checked={data.status}>
                        Status
                    </Checkbox>
                </Label>
            </div>
        </form>
    </div>

    <!-- Modal footer -->
    <div slot="footer">
        <Button on:click={submit}>
            {data.id ? "Save all" : "Add user"}
        </Button>
    </div>
</Modal>
