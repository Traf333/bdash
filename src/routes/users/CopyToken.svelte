<script>
    import { Button } from "flowbite-svelte";
    import { FileCopyOutline } from "flowbite-svelte-icons";
    // Content from the provided JavaScript file, escaped with String.raw``
    let content = `async function sendPostRequest(url, payload, token, expectJsonResponse = true) {
    const response = await fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: \`Bearer \${token}\`,
      },
      body: JSON.stringify(payload),
    });

    return expectJsonResponse ? response.json() : response.text();
  }

  async function balance(token) {
    const response = await fetch(
      "https://game-domain.blum.codes/api/v1/user/balance",
      {
        headers: {
          "Content-Type": "application/json",
          Authorization: \`Bearer \${token}\`,
        },
      },
    );
    const responseJson = await response.json();
    console.log("Balance of the account", responseJson);
    return responseJson.playPasses;
  }

  // Function to execute the play and claim cycle
  async function playAndClaimCycle(token) {
    let totalPoints = 0;
    let count = await balance(token);
    const max = Math.floor(Math.random() * (32 - 22 + 1)) + 22;
    count = Math.min(count, max);
    console.log("Number of games ", count);
    for (let i = 0; i < count; i++) {
      try {
        // First request to /api/v1/game/play
        const playResponse = await sendPostRequest(
          "https://game-domain.blum.codes/api/v1/game/play",
          {},
          token,
        );
        if (!playResponse.gameId) {
          continue;
        }
        console.log(\`Cycle \${i + 1} - Play response:\`, playResponse);

        // Wait for 35 seconds
        await new Promise((resolve) => setTimeout(resolve, 35000));

        // Generate random points between 150 and 189
        const points = Math.floor(Math.random() * (207 - 170 + 1)) + 170;
        console.log(\`Number of points \${points} will be claimed\`);

        // Second request to /api/v1/game/claim
        const claimResponse = await sendPostRequest(
          "https://game-domain.blum.codes/api/v1/game/claim",
          {
            gameId: playResponse.gameId, // Assuming the response contains the gameId
            points: points,
          },
          token,
          false, // Expecting a text response
        );
        console.log(\`Cycle \${i + 1} - Claim response:\`, claimResponse);
        totalPoints += points;
      } catch (error) {
        console.error(\`Cycle \${i + 1} - Error:\`, error);
        return;
      }
      await new Promise((resolve) => setTimeout(resolve, 5000));
    }

    console.log(\`TOTAL: \${totalPoints}\`);
  }
  function delay(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }

  // Start the play and claim cycle

  async function processAccounts(tokens) {
    for (let index = 0; index < tokens.length; index++) {
      const token = tokens[index];
      await delay(Math.random() * 1000 + 2000); // delay for 2-3 seconds
      playAndClaimCycle(token);
    }
  }`;

    export let accessToken = "";
    async function copyToClipboard() {
        const textToCopy = content + `processAccounts(["${accessToken}"])`;
        try {
            await navigator.clipboard.writeText(textToCopy);
            alert("Copied to clipboard!");
        } catch (err) {
            console.error("Failed to copy: ", err);
        }
    }
</script>

<Button size="sm" outline class="gap-2 px-3" on:click={copyToClipboard}>
    <FileCopyOutline size="sm" />
</Button>
