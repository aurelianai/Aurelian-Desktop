<script>
	import { clipboard } from '@skeletonlabs/skeleton';

	let sample_messages = [
		{ message: 'Who was Aurelian and why was he important?', copied: false },
		{ message: 'Explain the Special Theory of Relativity', copied: false },
		{
			message: 'Write an email to Sarah accepting an invitation to her party',
			copied: false
		}
	];
</script>

<div
	class="max-w-xl p-4 mx-auto shadow-xl rounded-xl bg-secondary-300-600-token card"
>
	<div class="flex flex-col items-center space-y-4">
		<p class="font-medium h6">
			You have no messages! Try one of these to start.
		</p>
		<div class="p-4 rounded-xl card">
			<ol class="space-y-4 list">
				{#each sample_messages as sample_message, i}
					<li>
						<span class="p-3 badge-icon variant-soft-primary">{i + 1}</span>
						<span data-clipboard={'example' + i.toString()} class="flex-auto">
							{sample_message.message}
						</span>
						{#if !sample_message.copied}
							<button
								use:clipboard={{ element: 'example' + i.toString() }}
								on:click={async () => {
									sample_messages[i].copied = true;
									sample_messages.forEach(
										(msg, index) => (msg.copied = i === index)
									);
								}}
								class="p-1 btn"
							>
								📋
							</button>
						{:else}
							<button class="p-1 btn"> ✅ </button>
						{/if}
					</li>
				{/each}
			</ol>
		</div>
	</div>
</div>
