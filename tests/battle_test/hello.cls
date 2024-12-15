class A {
  private static final List<String> CLASSES_TO_IGNORE {
    get {
      CLASSES_TO_IGNORE = new List<String>{'Class.' + Logger.class.getName(), 'Class.' + LogEntryEventBuilder.class.getName(), 'Class.' + LogMessage.class.getName()};
    }
    set;
  }

  {
    CLASSES_TO_IGNORE = new List<String>{'Class.' + Logger.class.getName(), 'Class.' + LogEntryEventBuilder.class.getName(), 'Class.' + LogMessage.class.getName()};
  }
}
