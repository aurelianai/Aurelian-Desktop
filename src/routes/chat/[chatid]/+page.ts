import type { PageLoad } from './$types'
import { getMessages } from '$lib/chat/ts'

export const load = (async ({ params }) => {
   let messages = await getMessages(+params.chatid)
   let chatid = +params.chatid

   return { chatid, messages }
}) satisfies PageLoad