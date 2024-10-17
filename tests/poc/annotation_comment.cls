class A {
  @TestVisible
  // comment1
  private String s;

  @TestVisible // comment2
  private String s;

  @TestVisible
  /* b_comment1 */
  private String s;

  @TestVisible
  /* b_comment2 */ private String s;
}
