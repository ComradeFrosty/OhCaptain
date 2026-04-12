<script>
  // onMount runs once when the component appears on screen
  // think of it like "when the widget wakes up, do this"
  import { onMount } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/core'; 

  // these are reactive variables - when they change, the UI updates automatically
  // that's one of Svelte's superpowers vs plain JS
  let quote = '';
  let author = '';
  let fetchedAt = '';
  let loading = true;

  // fetches a random quote from the Quotable API
  // async/await means "wait for the network, don't freeze the app"
  async function fetchQuote() {
    loading = true;

    try {
      const raw = await invoke('quotefetch');
      const data = JSON.parse(raw);
      quote = data[0].q;
      author = data[0].a;
    } catch (error) {
      // if the internet is down, show a fallback line
      quote = "The world is full of magic things, patiently waiting for our senses to grow sharper.";
      author = "W.B. Yeats";
    } finally {
      loading = false;
      fetchedAt = new Date().toLocaleTimeString();
    }
  }

  // when the widget mounts, fetch a quote immediately
  // then set a timer to refresh every 5 minutes (300,000 ms)

  function startDrag() {
  const appWindow = getCurrentWindow();
  appWindow.startDragging();
  }

  onMount(() => {
    fetchQuote();

    const interval = setInterval(fetchQuote, 300_000);
    

    // this return is a cleanup function - when the widget closes,
    // Svelte automatically calls this to stop the timer
    // without this, timers keep running in the background (memory leak)
    return () => clearInterval(interval);
  });
</script>

<!-- the widget card -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="widget" on:mousedown={startDrag}>
  {#if loading}
    <!-- Svelte's conditional rendering - shows this while fetching -->
    <p class="loading">...</p>
  {:else}
    <!-- once loaded, show the quote with a fade-in -->
    <div class="content">
      <p class="quote">"{quote}"</p>
      <p class="author">— {author}</p>
      <p class="fetched-at">{fetchedAt}</p> 
    </div>
  {/if}

  <!-- clicking the widget fetches a new quote immediately -->
  <button class="refresh" on:click={fetchQuote} on:mousedown|stopPropagation title="New quote">↻</button>
</div>

<style>
  /* removes default body margin/padding that would create white edges */
  :global(body) {
    margin: 0;
    padding: 0;
    background: transparent;
    overflow: hidden;
  }

  .fetched-at {
  font-size: 10px;
  opacity: 0.3;        /* even quieter than the author line */
  margin: 4px 0 0 0;
  letter-spacing: 0.03em;
  font-style: normal;  /* overrides italic inherited from parent */
  font-family: monospace; /* makes the time feel like a timestamp */
}

  .widget { 
    width: 380px;
    height: 160px;
    background: rgba(0, 0, 0, 0.55);  /* semi-transparent dark card */
    backdrop-filter: blur(12px);       /* frosted glass effect */
    border-radius: 0px;
    border: 1px solid rgba(255, 255, 255, 0.08);
    padding: 24px 28px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    justify-content: center;
    position: relative;
    color: white;
    font-family: 'Georgia', serif;    /* serif feels literary, fits the vibe */
  }

  .quote {
    font-size: 14px;
    line-height: 1.6;
    margin: 0 0 10px 0;
    opacity: 0.92;
    font-style: italic;
  }

  .author {
    font-size: 11px;
    opacity: 0.55;
    margin: 0;
    letter-spacing: 0.04em;
  }

  .loading {
    opacity: 0.4;
    font-size: 18px;
    text-align: center;
    margin: 0;
  }

  /* small refresh button in the corner, hidden until hover */
  .refresh {
    position: absolute;
    bottom: 10px;
    right: 14px;
    background: none;
    border: none;
    color: white;
    opacity: 0;
    cursor: pointer;
    font-size: 16px;
    transition: opacity 0.2s;
    padding: 4px;
  }

  /* show refresh button only when hovering the widget */
  .widget:hover .refresh {
    opacity: 0.4;
  }

  .widget:hover .refresh:hover {
    opacity: 1;
  }
</style>