<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    export let mnemonicPhrase;
    export let isCopied;
    export let copyToClipboard;
    export let dialog

    const dispatch = createEventDispatcher()
    function clickClose() {
        dispatch('closeDialog')
    }
</script>

<dialog bind:this={dialog}>
    <div class="dialog">
        <div class="card">
            <div class="card-content">
                <p class="key">Mnemonic Phrase:</p>
                <div class="mnemonic-wrapper">
                    <p class="mnemonic">{mnemonicPhrase}</p>
                </div>
                <button class="copy" on:click={() => copyToClipboard(mnemonicPhrase)}>Copy to Clipboard</button>
                <button on:click={clickClose}>Close</button>
                {#if isCopied}
                    <p class="copied-message">Copied!</p>
                {/if}
            </div>
        </div>
    </div>
</dialog>


<style>
    .dialog {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.4);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1000;
    }
    .card {
        background-color: white;
        padding: 20px;
        border-radius: 10px;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
        width: 550px;
        height: 420px;
    }
    .card-content {
        display: flex;
        flex-direction: column;
        align-items: center;
    }
    .mnemonic {
        color: var(--accent-color);
        width: 100%;
        margin-left: 2rem;
        margin-right: 2rem;
        margin-bottom: 2vh;
        font-size: 1.4rem;
        font-weight: bold;
        line-height: 2.0;
        overflow-wrap: break-word;
        text-align: left;
    }
    .mnemonic-wrapper {
        display: flex;
        justify-content: center;
        width: 100%;
    }
    .copy {
        margin-top: 2rem;
        margin-bottom: 2rem;
    }
    .copied-message {
        color: green;
        font-size: 1.2em;
        margin-top: 10px;
    }
</style>
