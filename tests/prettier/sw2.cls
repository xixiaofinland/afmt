public class Me {
  {
    Integer i = 0;
    switch on i {
      when 2, 3, 4 {
        System.debug('when block 2 and 3 and 4');
      }
      when 5, 6 {
        System.debug('when block 5 and 6');
      }
      when 7 {
        System.debug('when block 7');
      }
      when else {
        System.debug('default');
      }
    }
  }
}
