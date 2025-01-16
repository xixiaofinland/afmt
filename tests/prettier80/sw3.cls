public class Me {
  {
    Integer i = 0;

    switch on someInteger(i) {
      when 2 {
        System.debug('when block 2');
      }
      when 3 {
        System.debug('when block 3');
      }
      when else {
        System.debug('default');
      }
    }
  }
}
