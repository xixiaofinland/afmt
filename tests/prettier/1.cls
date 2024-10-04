class SafeNavigationOperator {
  public void t() {
    String s = [
      SELECT COUNT(Id), ApexClass.Name className, JobType jobType
      FROM AsyncApexJob
      GROUP BY ApexClass.Name, JobType
    ];
  }
}
