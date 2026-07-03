'use server'

import { db } from '@/lib/db'

// No session/ownership check at all before the mutation.
export async function deletePost(postId: string) {
  await db.post.delete({ where: { id: postId } })
}
