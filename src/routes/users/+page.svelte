<script lang="ts">
    import {
        Avatar,
        Breadcrumb,
        BreadcrumbItem,
        Button,
        Checkbox,
        Heading,
        Indicator,
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
    } from "flowbite-svelte-icons";
    import {
        EditOutline,
        ExclamationCircleSolid,
        PlusOutline,
        TrashBinSolid,
    } from "flowbite-svelte-icons";
    import { invoke } from "@tauri-apps/api/core";

    import User from "./User.svelte";
    import Delete from "./Delete.svelte";
    import ConfirmExport from "./ConfirmExport.svelte";
    import MetaTag from "../utils/MetaTag.svelte";
    let openUser: boolean = false; // modal control
    let openDelete: boolean = false; // modal control
    let search = "";
    let exportFile: File | undefined;
    import { users, toUserId } from "./store";
    import type { TUser } from "../../types";
    import { onMount } from "svelte";
    let current_user: TUser | undefined;
    const path: string = "/users";
    const description: string =
        "CRUD users examaple - Flowbite Svelte Admin Dashboard";
    const title: string = "Flowbite Svelte Admin Dashboard - CRUD Users";
    const subtitle: string = "CRUD Users";
    let openExport = false;
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
    $: filteredUsers = search
        ? $users.filter((u) =>
              u.name.toLowerCase().includes(search.toLowerCase()),
          )
        : $users;

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
            <div class="border-l border-gray-100 pl-2 dark:border-gray-700">
                <ToolbarButton
                    color="dark"
                    class="m-0 rounded p-1 hover:bg-gray-100 focus:ring-0 dark:hover:bg-gray-700"
                >
                    <CogSolid size="lg" />
                </ToolbarButton>
                <ToolbarButton
                    color="dark"
                    class="m-0 rounded p-1 hover:bg-gray-100 focus:ring-0 dark:hover:bg-gray-700"
                >
                    <TrashBinSolid size="lg" />
                </ToolbarButton>
                <ToolbarButton
                    color="dark"
                    class="m-0 rounded p-1 hover:bg-gray-100 focus:ring-0 dark:hover:bg-gray-700"
                >
                    <ExclamationCircleSolid size="lg" />
                </ToolbarButton>
                <ToolbarButton
                    color="dark"
                    class="m-0 rounded p-1 hover:bg-gray-100 focus:ring-0 dark:hover:bg-gray-700"
                >
                    <DotsVerticalOutline size="lg" />
                </ToolbarButton>
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
                <Button size="sm" color="alternative" class="gap-2 px-3">
                    <DownloadSolid size="md" class="-ml-1" /><label for="export"
                        >Export</label
                    >
                </Button>
                <input
                    type="file"
                    name="export"
                    id="export"
                    class="hidden"
                    on:change={handleFileChange}
                />
            </div>
        </Toolbar>
    </div>
    <Table striped hoverable>
        <TableHead
            class="border-y border-gray-200 bg-gray-100 dark:border-gray-700"
        >
            {#each ["Name", "Type", "Actions"] as title}
                <TableHeadCell class="p-4 font-medium">{title}</TableHeadCell>
            {/each}
        </TableHead>
        <TableBody>
            {#each filteredUsers as user}
                <TableBodyRow class="text-base">
                    <TableBodyCell class="p-4">
                        <div class="flex items-center gap-2">
                            <Indicator color={user.status ? "green" : "red"} />
                            {user.name}
                        </div>
                    </TableBodyCell>
                    <TableBodyCell class="p-4 font-normal">
                        {user.account_type}
                    </TableBodyCell>

                    <TableBodyCell class="w-2 space-x-2 p-4">
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
<ConfirmExport bind:open={openExport} />
