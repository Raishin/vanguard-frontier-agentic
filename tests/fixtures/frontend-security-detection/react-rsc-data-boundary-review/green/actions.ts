'use server'

import { auth } from '@/lib/auth'
import { db } from '@/lib/db'

export async function deletePost(postId: string) {
  const session = await auth()
  if (!session?.user) {
    throw new Error('Unauthorized')
  }

  const post = await db.post.findUnique({ where: { id: postId } })

  // Check that the user owns this resource before mutating it.
  if (post.authorId !== session.user.id) {
    throw new Error('Forbidden')
  }

  await db.post.delete({ where: { id: postId } })
}
