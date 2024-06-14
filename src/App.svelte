<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import bs58 from 'bs58';
	import Header from './Header.svelte';
	import { onMount } from 'svelte';

	let providedCode = '';
	let passwordString = '';
	let bitcoinPrivateKey = '';
	let mnemonicPhrase = '';
	let error = '';
	let warning = '';
	let isPasswordVisible = true;
	let isButtonDisabled = true;
    let loader = document.getElementById('loader');
	let mnemonicDisplay = document.getElementById('mnemonicDisplay');
	let isCopied = false;


	function providedCodeInput() {
		if (providedCode.length < 16) {
			warning += 'The provided code must be at least 16 characters long';
		}
	}

	function passwordStringInput() {
		if (passwordString.length < 8) {
			warning += 'The password must be at least 8 characters long';
		}
	}

	async function copyToClipboard(text: string) {
		try {
      await navigator.clipboard.writeText(text);

      isCopied = true; // コピー完了メッセージを表示
    } catch (err) {
      console.error('error copying to clipboard ', err);
    }
	}

	async function handleFormSubmit() {
		validateInput(); // Base58形式の検証
		providedCodeInput(); // Provided codeの長さ検証
		passwordStringInput(); // Passwordの長さ検証

		// 他の条件が満たされている場合のみ、バックエンドの関数を呼び出す
		if (warning === '') {
			try {
				if (loader !== null) {
                   loader.style.display = 'block'; // ローダーを表示
                }
				const result = await invoke<[string, string]>('generate_hd_wallet_xprv', { providedCode, passwordString });
				if (loader !== null) {
                    loader.style.display = 'none'; // ローダーを非表示
                }
				if (Array.isArray(result) && result.length === 2) {
					bitcoinPrivateKey = result[0]; // コンポーネントのステートを更新
					mnemonicPhrase = result[1];
					if (mnemonicDisplay !== null) {
                        mnemonicDisplay.innerText = mnemonicPhrase;
                    }
				} else {
					console.error('Result is not the expected format');
				}
			} catch (err) {
				if(loader){
					loader.style.display = 'none';
				}
				error = (err as Error).message;
			}
		}
	}

	// 入力検証を行う関数
	function validateInput() {
		// Base58形式の検証
		let isBase58Valid = true;
		try {
			bs58.decode(providedCode);
			bs58.decode(passwordString);
			warning = '';
		} catch (e) {
			warning = 'Please enter the Base58 characters';
			isBase58Valid = false;
		}

		// Provided codeとPasswordの長さ検証
		if (providedCode.length < 16) {
			warning += 'The provided code must be at least 16 characters long';
			isButtonDisabled = true;
		} else if (passwordString.length < 8) {
			warning += 'The password must be at least 8 characters long';
			isButtonDisabled = true;
		} else if (isBase58Valid) {
			isButtonDisabled = false; // すべての条件を満たす場合、ボタンを活性化
		}
	}

	function generateKeys() {
		handleFormSubmit();
	}

	  onMount(() => {
    loader = document.getElementById('loader');
    mnemonicDisplay = document.getElementById('mnemonicDisplay');
  });

</script>


<Header title="Block Hand Bitcoin" />

<main class="background">

	<div class="container">
		<div class="moveUp2"></div>

		<h2>Let's start use Bitcoin mnemonic generator</h2>
		<h4>Block Hand Bitcoin is an open-source application that generates mnemonic phrases.<br>
			By combining the string engraved on your accessory with your own unique password, you can manage the mnemonics of your Bitcoin wallet.</h4>
		<h3>
			Please enter a password using characters other than
			+, /, 0 (zero), O (uppercase 'o'), I (uppercase 'i'),
			and l (lowercase 'L') by Base58
		</h3>

		<div class="loader"></div>
		<div class="password-input">
			{#if isPasswordVisible}
				<input
					class="input-field"
					bind:value={providedCode}
					type="text"
					placeholder="Input cord inside ring"
					on:input={validateInput}
				/>
			{:else}
				<input
					class="input-field"
					bind:value={providedCode}
					type="password"
					placeholder="Input cord inside ring"
					on:input={validateInput}
				/>
			{/if}
		</div>

		<div class="password-input">
			{#if isPasswordVisible}
				<input
				    class="input-field"
					bind:value={passwordString}
					type="text"
					placeholder="Input password of at least 8 characters"
					on:input={validateInput}
				/>
			{:else}
				<input
				    class="input-field"
					bind:value={passwordString}
					type="password"
					placeholder="Input password of at least 8 characters"
					on:input={validateInput}
				/>
			{/if}
		</div>

		<button on:click={generateKeys} disabled={isButtonDisabled}>Generate Keys</button>

		{#if warning}
			<p class="warning">{warning}</p>
		{/if}


		{#if mnemonicPhrase}
			<p class="key">Mnemonic Phrase:</p>
			<p class="mnemonic">{mnemonicPhrase}</p>
			<button on:click={() => copyToClipboard(mnemonicPhrase)}>Copy to Clipboard</button>
			 {#if isCopied}
        <p class="copied-message">Copied!</p>
      {/if}
		{/if}

		{#if error}
			<p class="error">{error}</p>
		{/if}
	</div>
	<h4>©︎Vaultwear</h4>
</main>

<style>
	:root {
		--primary-color: #aca99b;
		--secondary-color: #d8d4ce;
		--background-color: #e8e8e9;
		--base-color: #c0bcaf;
		--accent-color: #4f4f47;
	}


	.background {

		background-image: linear-gradient(
			to right,
		); /* グラデーション適用 */
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		border-radius: 10px;
		display: flex; /* Flexboxを有効にします */
		flex-direction: column; /* 子要素を縦方向に配置します */
		justify-content: center; /* 子要素を水平方向で中央に配置します */
		align-items: center; /* 子要素を垂直方向で中央に配置します */
		 position: absolute;
	}

	.container {
		display: flex; /* Flexboxを有効にします */
		flex-direction: column; /* 子要素を縦方向に配置します */
		justify-content: center; /* 子要素を水平方向で中央に配置します */
		align-items: center; /* 子要素を垂直方向で中央に配置します */
		margin: 0 auto; /* ウィンドウ全体を中央に配置します */
		font-family: 'Courier New', monospace;
		width: 100%;
        margin-bottom: 10vh;

	}


  .moveUp2 {
		position: absolute;
		right: -30px;
        bottom: -100px; /* アニメーション開始位置 */
        width: 470px; /* 画像の幅を調整 */
        height: 470px; /* 画像の高さを調整 */
        background-image: url('/img/bitcoin-symbol.svg');
        background-size: contain;
        background-repeat: no-repeat;
        animation: moveUp2 25s linear infinite, rotate2 19s linear infinite;
        animation-delay: 10s;
		z-index: -1;
        opacity: 0;
    }

	@keyframes moveUp2 {
    0% {
      bottom: -100px; /* アニメーション開始位置 */
	opacity: 0;
    }
	50% {
      opacity: .5; /* 70%の高さまでは完全に表示 */
    }
    100% {
      bottom: 100vh; /* 画面の高さ */
	  opacity: 0;
    }
  }
   @keyframes rotate2 {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }




     h2 {
		margin-top: 6rem;
		margin-bottom: 3rem;
		font: 2rem "Fira Sans", sans-serif, bold;
		color: #51b24d;
	 }
	 h3{
		font-size: 1rem;
		margin-bottom: 3rem;
		margin-left: 10rem;
		margin-right: 10rem;
       color: var(--accent-color);
	   text-align: left;
	 }
	 h4{
		width: 60%;
color: var(--accent-color);
margin-bottom: 4rem;
   text-align: left;
	 }


	input {
		width: 80%;
		padding: 10px;
		margin-bottom: 20px;
		background-color: transparent;
		border: none;
		border-bottom: 2px solid #8e8c81;
		font-size: 1rem;
		color: #8e8c81;
	}

	input::placeholder {
		color: #aca99b;
	}

	input:focus {
    border-color: #aca99b; /* フォーカス時のボーダー色を変更 */
    box-shadow: 0 0 3px #aca99b; /* フォーカス時のシャドウを追加 */
}


	.warning,
	.error {
		color: accent-color;
		margin-top: 1rem;
	}

	button {
    background-color: var(--primary-color);
    color: #ffffff;
    padding: 10px 15px;
    border: none;
    border-radius: 5px;
    transition: background-color 0.3s ease;
	 margin-bottom: 20px;
    background-color: #51b24d;
}


	button:disabled {
    background-color: #838383;
    cursor: not-allowed;
}

	button:hover {
    background-color: #F7931A; /* ホバー時の背景色を暗く */
}

   .password-input{
   width: 90%;
   display: flex;
   justify-content: center;
   align-items: center;
   margin-bottom: 1.5rem;

   }
   .input-field{
	background-color:#e0ddd8}

	.key{
		color: var(--accent-color);
		width: 80%;
		text-align: left;

	}
	.mnemonic{
		color: var(--accent-color);
		width: 80%;
		text-align: left;
		margin-bottom: 2vh;
	}

	.copied-message {
    color: green;
    font-size: 1.2em;
    margin-top: 10px;
  }

	/* HTML: <div class="loader"></div> */
.loader {
      width: 90px;
      height: 14px;
      box-shadow: 0 3px 0 #51b24d;
      position: relative;
      clip-path: inset(-40px 0 -5px);
      display: none; /* 初期状態で非表示 */
    }
.loader:before {
  content: "";
  position: absolute;
  inset: auto calc(50% - 17px) 0;
  height: 50px;
  --g:no-repeat linear-gradient(#F7931A 0 0);
  background: var(--g),var(--g),var(--g),var(--g);
  background-size: 16px 14px;
  animation:
    l7-1 2s infinite linear,
    l7-2 2s infinite linear;
}
@keyframes l7-1 {
  0%,
  100%  {background-position: 0 -50px,100% -50px}
  17.5% {background-position: 0 100%,100% -50px,0 -50px,100% -50px}
  35%   {background-position: 0 100%,100% 100% ,0 -50px,100% -50px}
  52.5% {background-position: 0 100%,100% 100% ,0 calc(100% - 16px),100% -50px}
  70%,
  98%  {background-position: 0 100%,100% 100% ,0 calc(100% - 16px),100% calc(100% - 16px)}
}
@keyframes l7-2 {
  0%,70% {transform:translate(0)}
  100%  {transform:translate(200%)}
}
</style>

