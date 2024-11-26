class MapList {
  {
    Map<String, String> a = new Map<String, String>{
      'Content' => 'Hello',
      'World' => 'There',
      'Hi' => 'Yes'
    };

    JSON.serialize(
      new Map<String, Object>{
        'Content' => 'Hello',
        'World' => 'There',
        'Hi' => 'Yes'
      }
    );

    JSON.serialize(
      t.a('1111111111111111111111111111111111111111111111111111111111111111111111111111')
    );
  }
}
