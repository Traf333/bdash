<script>
    import { onMount } from "svelte";
    // Example data
    import { invoke } from "@tauri-apps/api/core";
    import { users } from "./users/store";
    $: totalBalance = Math.round(
        $users.reduce((acc, u) => acc + (u.data?.total_balance || 0), 0),
    );
    $: totalAvailablePasses = $users.reduce(
        (acc, u) => acc + (u.data?.play_passes || 0),
        0,
    );
    $: numberOfUsers = $users.length;
    onMount(() => {
        invoke("accounts").then((d) => users.set(d));
    });
</script>

<div class="grid grid-cols-1 sm:grid-cols-3 gap-4 p-4">
    <div class="card">
        <h2 class="text-xl font-bold">Total Balance</h2>
        <p class="text-4xl text-blue-500">{totalBalance.toLocaleString()}</p>
    </div>

    <div class="card">
        <h2 class="text-xl font-bold">Available Passes</h2>
        <p class="text-4xl text-green-500">
            {totalAvailablePasses.toLocaleString()}
        </p>
    </div>

    <a class="card card-clickable" href="/users">
        <h2 class="text-xl font-bold">Number of Users</h2>
        <p class="text-4xl text-purple-500">{numberOfUsers.toLocaleString()}</p>
    </a>
</div>

<style>
    .card {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 1rem;
        border: 1px solid #e5e7eb;
        border-radius: 0.5rem;
        box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
        transition: box-shadow 0.3s ease-in-out;
    }

    .card:hover {
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    }

    .card-clickable {
        cursor: pointer;
    }
</style>
