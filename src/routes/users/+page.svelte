<script lang="ts">
    import {
        Avatar,
        Breadcrumb,
        BreadcrumbItem,
        Button,
        Checkbox,
        Heading,
        Indicator,
        Label,
        Select,
    } from "flowbite-svelte";
    import {
        Input,
        Table,
        TableBody,
        TableBodyCell,
        TableBodyRow,
        TableHead,
    } from "flowbite-svelte";
    import {
        TableHeadCell,
        Toolbar,
        ToolbarButton,
        ToolbarGroup,
    } from "flowbite-svelte";
    import {
        CogSolid,
        DotsVerticalOutline,
        DownloadSolid,
        FileCopyOutline,
        FilterSolid,
    } from "flowbite-svelte-icons";
    import {
        EditOutline,
        ExclamationCircleSolid,
        PlusOutline,
        RefreshOutline,
        TrashBinSolid,
    } from "flowbite-svelte-icons";
    import { invoke } from "@tauri-apps/api/core";

    import User from "./User.svelte";
    import Delete from "./Delete.svelte";
    import MetaTag from "../utils/MetaTag.svelte";
    let openUser: boolean = false; // modal control
    let openDelete: boolean = false; // modal control
    let search = "";
    let selectedStatus: boolean | string = "all";
    let sortColumn: "name" | "balance" | "passes" = "name";
    let sortOrder: "asc" | "desc" = "desc";
    import { users, toUserId } from "./store";
    import type { TUser } from "../../types";
    import { onMount } from "svelte";
    import CopyToken from "./CopyToken.svelte";
    import { copyToClipboard } from "../utils/copy";
    let current_user: TUser | undefined;
    const path: string = "/users";
    const description: string =
        "CRUD users examaple - Flowbite Svelte Admin Dashboard";
    const title: string = "Flowbite Svelte Admin Dashboard - CRUD Users";
    const subtitle: string = "CRUD Users";
    let openExport = false;
    let exportFile;
    function handleFileChange(event: Event) {
        openExport = true;
        console.log(typeof event);
        const target = event.target as HTMLInputElement;
        const file = target.files && target.files[0];
        if (file) {
            exportFile = file;
            // Add your file handling logic here, such as uploading or reading the file.
            console.log(`Selected file: ${file.name}`);
        }
    }

    async function refresh(user: TUser) {
        let response = await invoke<TUser>("refresh_account", {
            id: toUserId(user.id),
        });
        console.log(response, "response");
        users.updateUser(response);
    }
    function sortUsers(users: TUser[]): TUser[] {
        return [...users].sort((a, b) => {
            if (isNaN(Number(a.name)) || isNaN(Number(b.name))) return -1;
            return Number(a.name) > Number(b.name) ? 1 : -1;
        });
    }

    $: filteredUsers = $users.filter((u) => {
        const matchesSearch = search
            ? u.name.toLowerCase().includes(search.toLowerCase())
            : true;
        const matchesStatus =
            typeof selectedStatus === "boolean"
                ? u.status === selectedStatus
                : true;
        return matchesSearch && matchesStatus;
    });
    let selectAll = false;
    function toggleSelectAll() {
        // selectAll = !selectAll;
        filteredUsers.forEach((user) => {
            user.selected = selectAll;
        });
        users.set($users); // Update the users store
    }

    onMount(async () => {
        users.set(await invoke("accounts"));
    });
</script>

<MetaTag {path} {description} {title} {subtitle} />

<main class="relative h-full w-full overflow-y-auto bg-white dark:bg-gray-800">
    <div class="p-4">
        <Breadcrumb class="mb-5">
            <BreadcrumbItem home href="/">Home</BreadcrumbItem>
            <BreadcrumbItem href="/users">Users</BreadcrumbItem>
            <BreadcrumbItem>List</BreadcrumbItem>
        </Breadcrumb>
        <Heading
            tag="h1"
            class="text-xl font-semibold text-gray-900 dark:text-white sm:text-2xl"
        >
            All users
        </Heading>
        <Toolbar embedded class="w-full py-4 text-gray-500  dark:text-gray-400">
            <Input
                placeholder="Search for users"
                class="me-4 w-80 border xl:w-96"
                bind:value={search}
            />
            <Label>
                <Select
                    class="mr-2"
                    items={[
                        { value: "all", name: "Все" },
                        { value: true, name: "Активные" },
                        { value: false, name: "Забанненые" },
                    ]}
                    bind:value={selectedStatus}
                />
            </Label>
            <div class="border-l border-gray-100 pl-2 dark:border-gray-700">
                <ToolbarButton
                    color="dark"
                    class="m-0 rounded p-1 hover:bg-gray-100 focus:ring-0 dark:hover:bg-gray-700"
                    on:click={() => filteredUsers.map(refresh)}
                >
                    <RefreshOutline size="lg" />
                </ToolbarButton>
                <ToolbarButton
                    color="dark"
                    class="m-0 rounded p-1 hover:bg-gray-100 focus:ring-0 dark:hover:bg-gray-700"
                    on:click={() =>
                        copyToClipboard(
                            filteredUsers
                                .filter((u) => u.selected)
                                .map((u) => u.access_token),
                        )}
                >
                    <FileCopyOutline size="lg" />
                </ToolbarButton>
                <!-- <ToolbarButton
                    color="dark"
                    on:click={() =>
                        alert(
                            JSON.stringify(
                                filteredUsers.map((f) => f.access_token),
                            ),
                        )}
                    class="m-0 rounded p-1 hover:bg-gray-100 focus:ring-0 dark:hover:bg-gray-700"
                >
                    <ExclamationCircleSolid size="lg" />
                </ToolbarButton> -->
                <!-- <ToolbarButton
                    color="dark"
                    class="m-0 rounded p-1 hover:bg-gray-100 focus:ring-0 dark:hover:bg-gray-700"
                >
                    <DotsVerticalOutline size="lg" />
                </ToolbarButton> -->
            </div>

            <div slot="end" class="flex items-center space-x-2">
                <Button
                    size="sm"
                    class="gap-2 whitespace-nowrap px-3"
                    on:click={() => (
                        (current_user = undefined), (openUser = true)
                    )}
                >
                    <PlusOutline size="sm" />Add user
                </Button>
                <!-- <Button size="sm" color="alternative" class="gap-2 px-3">
                    <DownloadSolid size="md" class="-ml-1" /><label for="export"
                        >Export</label
                    >
                </Button> -->
                <!-- <input
                    type="file"
                    name="export"
                    id="export"
                    class="hidden"
                    on:change={handleFileChange}
                /> -->
            </div>
        </Toolbar>
    </div>
    <Table striped hoverable>
        <TableHead
            class="border-y border-gray-200 bg-gray-100 dark:border-gray-700"
        >
            <TableHeadCell class="w-4 p-4"
                ><Checkbox
                    on:change={toggleSelectAll}
                    bind:checked={selectAll}
                /></TableHeadCell
            >
            {#each ["Name", "Balance", "Passes", "Actions"] as title}
                <TableHeadCell class="p-4 font-medium">{title}</TableHeadCell>
            {/each}
        </TableHead>
        <TableBody>
            {#each sortUsers(filteredUsers) as user}
                <TableBodyRow class="text-base">
                    <TableBodyCell class="w-4 p-4"
                        ><Checkbox
                            bind:checked={user.selected}
                        /></TableBodyCell
                    >
                    <TableBodyCell class="p-4">
                        <div class="flex items-center gap-2">
                            <Indicator color={user.status ? "green" : "red"} />
                            {user.name}
                        </div>
                    </TableBodyCell>
                    <TableBodyCell class="p-4 font-normal">
                        {user.data?.total_balance}
                    </TableBodyCell>
                    <TableBodyCell class="p-4 font-normal">
                        {user.data?.play_passes}
                    </TableBodyCell>

                    <TableBodyCell class="w-2 space-x-2 p-4">
                        <CopyToken accessToken={user.access_token} />

                        <Button
                            size="sm"
                            class="gap-2 px-3"
                            on:click={() => (
                                (current_user = user), (openUser = true)
                            )}
                        >
                            <EditOutline size="sm" />
                        </Button>
                        <Button
                            size="sm"
                            class="gap-2 px-3"
                            on:click={() => refresh(user)}
                        >
                            <RefreshOutline size="sm" />
                        </Button>
                        <Button
                            color="red"
                            size="sm"
                            class="gap-2 px-3"
                            on:click={() => (
                                (current_user = user), (openDelete = true)
                            )}
                        >
                            <TrashBinSolid size="sm" />
                        </Button>
                    </TableBodyCell>
                </TableBodyRow>
            {/each}
        </TableBody>
    </Table>
</main>

<!-- Modals -->

<User bind:open={openUser} data={current_user} />
{#if current_user}
    <Delete bind:open={openDelete} id={current_user?.id?.id?.String || ""} />
{/if}
<!-- <ConfirmExport bind:open={openExport} /> -->
