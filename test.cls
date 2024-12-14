global class PluginDescribeResult {
  global class InputParameter {
    global String description;
    global String name;
    global Process.PluginDescribeResult.ParameterType parameterType;
    global Boolean required;
    global PluginDescribeResult.InputParameter(
      String name,
      Process.PluginDescribeResult.ParameterType parameterType,
      Boolean required
    ) {
    }
    global Object clone() {
    }
  }
  global class OutputParameter {
    global String description;
    global String name;
    global Process.PluginDescribeResult.ParameterType parameterType;
    global Object clone() {
    }
  }
  global enum ParameterType {
    BOOLEAN,
    DATE,
    DATETIME,
    DECIMAL,
    DOUBLE,
    FLOAT,
    ID,
    INTEGER,
    LONG,
    STRING
  }
}

