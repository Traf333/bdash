<script lang="ts">
    import { Button } from "flowbite-svelte";
    import { FileCopyOutline } from "flowbite-svelte-icons";
    import { copyToClipboard } from "../utils/copy";

    export let accessToken = "";
</script>

<Button
    size="sm"
    outline
    class="gap-2 px-3"
    on:click={() => copyToClipboard([accessToken])}
>
    <FileCopyOutline size="sm" />
</Button>
