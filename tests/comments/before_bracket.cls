// 1. wrong code node association?
// 2. block comment has one extra trailing space.
trigger InlineTrailingCommentAfterTrigger on Account /* Block Comment */(
  before insert,
  after insert
) {
}
