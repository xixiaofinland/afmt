// 1. wrong code node association?
// 2. block comment has one extra trailing space.
trigger InlineTrailingCommentAfterTrigger on Account(before insert, after insert)  /* Block Comment */ {
}
