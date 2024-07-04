<script>
    import "../app.css";
    import Footer from "./Footer.svelte";
</script>

<div class="min-h-screen flex flex-col justify-between">
    <div>
        <slot></slot>
    </div>
    <footer class="w-full">
        <Footer />
    </footer>
</div>
