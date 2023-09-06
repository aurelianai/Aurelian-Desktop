import type { PageLoad } from './$types'
import { getMessages } from '..'

export const load = (async ({ params }) => {
   let messages = await getMessages(+params.chatid)
   let chatid = +params.chatid

   return { chatid, messages }
}) satisfies PageLoad