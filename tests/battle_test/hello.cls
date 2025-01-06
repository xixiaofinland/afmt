class A {
    {

          // it would have been nice if this was an enum!
    switch on header.changeType {
      when 'CREATE', 'UNDELETE' {
        apexContext = TriggerOperation.AFTER_INSERT;
      }
      when 'UPDATE' {
        apexContext = TriggerOperation.AFTER_UPDATE;
      }
      when 'DELETE' {
        apexContext = TriggerOperation.BEFORE_DELETE;
      }
    }
}
}
