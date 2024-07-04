<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Alert } from "flowbite-svelte";

  let name = "";
  let greetMsg = "";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
  }
</script>

<div class="container mx-auto">
  <div class="flex flex-col sm:flex-row gap-4">
    <div
      class="flex-1 shadow-lg bg-white dark:bg-gray-800 shadow-gray-300 dark:shadow-gray-700 rounded-lg"
    >
      <div class="py-4 px-6">
        <div class="flex items-center justify-between">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
            Total Balance
          </h3>
          <a
            href="/users"
            class="text-sm font-medium text-blue-600 dark:text-blue-500 hover:underline"
            >Users</a
          >
        </div>
        <div class="mt-4 text-5xl font-bold text-gray-900 dark:text-white">
          1,345,678
        </div>
      </div>
    </div>

    <div
      class="flex-1 shadow-lg bg-white dark:bg-gray-800 shadow-gray-300 dark:shadow-gray-700 rounded-lg"
    >
      <div class="py-4 px-6">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
          Total Passes
        </h3>
        <div class="mt-4 text-5xl font-bold text-gray-900 dark:text-white">
          2,303
        </div>
      </div>
    </div>

    <div
      class="flex-1 shadow-lg bg-white dark:bg-gray-800 shadow-gray-300 dark:shadow-gray-700 rounded-lg"
    >
      <div class="py-4 px-6">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
          Number of Users
        </h3>
        <div class="mt-4 text-5xl font-bold text-gray-900 dark:text-white">
          100
        </div>
      </div>
    </div>
  </div>
</div>
