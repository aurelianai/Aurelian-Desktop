import type { Chat, Message } from './types'
import { writable } from 'svelte/store'

export const ChatStore = writable<Chat[]>()

export const getChats = async (): Promise<Chat[]> => {
   return [
      {
         id: 1,
         title: "Go Generics"
      },
      {
         id: 2,
         title: "Javascript HTTP"
      }
   ]
}

export const newChat = async (title: string): Promise<Chat> => {
   return {
      id: 1,
      title
   }
}

export const updateChat = async (id: number, new_name: string) => {
   console.log("updated chat ", id, "with new name", new_name)
   return
}

export const deleteChat = async (id: number) => {
   console.log("delted chat", id)
   return
}

export const getMessages = async (id: number): Promise<Message[]> => {
   if (id === 1) {
      return [
         {
            role: "USER",
            content: "Hello!"
         },
         {
            role: "MODEL",
            content: "Hi!, How's it going?"
         }
      ]
   } else {
      return [
         {
            role: "USER",
            content: "How do I write python!"
         },
         {
            role: "MODEL",
            content: "Follow tuts, then jump in!"
         }
      ]
   }
}

export const newMessage = async (chatId: number, role: "USER" | "MODEL", content: string): Promise<Message> => {
   console.log("creating new message for chat", chatId, "with role", role, "with content", content)
   return {
      role,
      content,
   }
}

export type InferenceUpdate = {
   delta: string,
   err: string | null,
}

export async function* complete(id: number, sig: AbortSignal): AsyncGenerator<InferenceUpdate> {
   for (let i = 0; i < 10; i++) {
      if (sig.aborted) { return }
      yield {
         delta: "hi ",
         err: null,
      }
   }
}