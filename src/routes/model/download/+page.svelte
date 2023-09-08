<script lang="ts">
	import { open } from '@tauri-apps/api/shell';
	import {
		Icon,
		ArrowLeft,
		ArrowDownTray,
		Heart,
		Bookmark
	} from 'svelte-hero-icons';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<a
	href="/model"
	class="fixed w-20 p-2 space-x-2 font-bold rounded-md btn variant-filled-primary top-20 left-5"
>
	<span><Icon src={ArrowLeft} class="w-5 h-5" /></span>
	<span>Back</span>
</a>

<div class="fixed bottom-0 right-0 w-full h-full top-36">
	<div class="max-w-md mx-auto rounded-md shadow card bg-surface-50-900-token">
		<header class="space-y-3 card-header">
			<div class="text-xl">
				<button
					class="text-tertiary-800-100-token hover:anchor"
					on:click={async () => {
						await open(`https://huggingface.co/${data.id.split('/')[0]}`);
					}}
				>
					{data.id.split('/')[0]}
				</button>
				/
				<button
					class="hover:anchor"
					on:click={async () => {
						await open(`https://huggingface.co/${data.id}`);
					}}
				>
					{data.id.split('/')[1]}
				</button>
			</div>

			<div class="grid grid-cols-3">
				<div class="flex items-center justify-center space-x-2">
					<span><Icon src={ArrowDownTray} class="w-4 h-4" /></span>
					<span class="font-semibold">{data.downloads.toLocaleString()}</span>
				</div>
				<div class="flex items-center justify-center space-x-2">
					<span><Icon src={Heart} class="w-4 h-4" /></span>
					<span class="font-semibold">{data.likes.toLocaleString()}</span>
				</div>
				<div class="flex items-center justify-center space-x-2">
					<span><Icon src={Bookmark} class="w-4 h-4" /></span>
					<span class="font-semibold">{data.cardData.license}</span>
				</div>
			</div>
			<hr class="opacity-100" />
		</header>

		<section class="p-4 space-y-4">
			{#each data.siblings as file}
				<span>
					{file.rfilename}
				</span>
			{/each}
		</section>
	</div>
</div>
